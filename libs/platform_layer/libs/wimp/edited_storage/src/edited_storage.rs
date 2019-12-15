use platform_types::*;

use rand::{thread_rng, Rng};
use shared::{BufferStatus, BufferStatusMap};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn store_buffers(
    edited_files_dir: &Path,
    edited_files_index_path: &Path,
    all_buffers: Vec<BufferView>,
    index_state: g_i::State,
    buffer_statuses: &mut BufferStatusMap,
) -> std::io::Result<()> {
    let mut rng = thread_rng();

    let index_string = std::fs::read_to_string(edited_files_index_path).unwrap_or_default();
    let mut names_to_uuid: HashMap<BufferName, u128> =
        HashMap::with_capacity(index_string.lines().count());

    for line in index_string.lines() {
        if let Some((name, uuid)) = deserialize(line) {
            names_to_uuid.insert(name, uuid);
        }
    }

    for (i, buffer) in all_buffers.into_iter().enumerate() {
        let filename = if let Some(uuid) = names_to_uuid.get(&buffer.name) {
            get_path(buffer.name_string, uuid)
        } else {
            let uuid: u128 = rng.gen();

            let path = get_path(buffer.name_string, &uuid);

            // we don't expect to read this again in the same
            // loop, but it should be saved back to disk for
            // next time.
            names_to_uuid.insert(buffer.name, uuid);

            path
        };

        // TODO replace all files in directory with these files atomically if possible
        std::fs::write(edited_files_dir.join(filename), buffer.data.chars)?;

        let index = index_state.new_index(g_i::IndexPart::or_max(i));

        if let Some(BufferStatus::EditedAndUnSaved) = buffer_statuses.get(index_state, index) {
            buffer_statuses.insert(index_state, index, BufferStatus::EditedAndSaved);
        }
    }

    let mut index_string = String::with_capacity(names_to_uuid.len() * INDEX_LINE_LENGTH_ESTIMATE);

    for (k, v) in names_to_uuid {
        serialize(&k, v, &mut index_string);
        index_string.push('\n');
    }

    Ok(())
}

fn get_path(buffer_name: String, uuid: &u128) -> PathBuf {
    let slug = buffer_name.replace(|c: char| !c.is_ascii_alphabetic(), "_");

    PathBuf::from(format!("{}_{}", slug, uuid))
}

const PATH_PREFIX: &'static str = "Path: ";
const SCRATCH_PREFIX: &'static str = "Scratch: ";
const SCRATCH_PREFIX_LENGTH: usize = 9;

const UUID_SUFFIX_LENGTH: usize = 32
    + 1 // for comma
;

const PATH_LENGTH_ESTIMATE: usize = 128;

const INDEX_LINE_LENGTH_ESTIMATE: usize =
    SCRATCH_PREFIX_LENGTH // the longest prefix
    + UUID_SUFFIX_LENGTH
    + PATH_LENGTH_ESTIMATE // the longest extra field
    ;

fn serialize(name: &BufferName, uuid: u128, append_target: &mut String) {
    use std::fmt::Write;
    use BufferName::*;
    let _write_for_string_always_works = match name {
        Path(p) => {
            // if a user selects a non-unicode path, then at least this way the data will be
            // preserved, even if it might be directed to a different filename. Excluding the
            // corner case of two distinct paths being lossily converted to the same string,
            // which would result in the most recent one overwriting the other one I guess?
            // Hopefully seeing the first file being rendered with replacement characters will
            // convince you to be careful editing files with those filenames?
            let path_string = p.to_string_lossy();
            write!(
                append_target,
                "{}{:032x},{}",
                PATH_PREFIX, uuid, path_string
            )
        }
        Scratch(n) => write!(append_target, "{}{:032x},{}", SCRATCH_PREFIX, uuid, n),
    };
}

fn deserialize(s: &str) -> Option<(BufferName, u128)> {
    if s.starts_with(PATH_PREFIX) {
        // works becasue the prefix is ASCII
        let s = &s[PATH_PREFIX.len()..];

        if let Some((uuid, path)) = split_off_uuid_and_comma(s) {
            return Some((BufferName::Path(PathBuf::from(path)), uuid));
        }
    } else if s.starts_with(SCRATCH_PREFIX) {
        // works becasue the prefix is ASCII
        let s = &s[SCRATCH_PREFIX.len()..];
        if let Some((uuid, rest)) = split_off_uuid_and_comma(s) {
            if let Ok(n) = rest.parse() {
                return Some((BufferName::Scratch(n), uuid));
            }
        }
    }

    None
}

fn split_off_uuid_and_comma<'s>(s: &'s str) -> Option<(u128, &'s str)> {
    // > not >= to exclude "" as a `rest`!
    if s.chars().count() > UUID_SUFFIX_LENGTH {
        let (uuid_and_comma, rest) = &s.split_at(UUID_SUFFIX_LENGTH);
        if uuid_and_comma.ends_with(",") {
            if let Ok(uuid) = u128::from_str_radix(&uuid_and_comma[..(UUID_SUFFIX_LENGTH - 1)], 16)
            {
                return Some((uuid, rest));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{arbitrary::any, prop_oneof, proptest, strategy::Strategy};

    pub fn buffer_name() -> impl Strategy<Value = BufferName> {
        prop_oneof![
            any::<u32>().prop_map(BufferName::Scratch),
            any::<std::ffi::OsString>().prop_map(|s| {
                let s = if s.len() == 0 {
                    "empty-paths-are-invalid".into()
                } else {
                    s
                };
                BufferName::Path(PathBuf::from(s))
            })
        ]
    }

    fn serialize_then_deserialize_works_on(name: BufferName, uuid: u128) {
        let mut serialized = String::with_capacity(UUID_SUFFIX_LENGTH);
        serialize(&name, uuid, &mut serialized);
        let deserialized = deserialize(&serialized);
        let expected = Some((name, uuid));
        assert_eq!(
            deserialized, expected,
            "{:?} deserialized into \"{:?}\", not \"{:?}\"",
            serialized, deserialized, expected
        )
    }

    proptest! {
        #[test]
        fn serialize_then_deserialize_works(
            name in buffer_name(),
            uuid in any::<u128>(),
        ) {
            serialize_then_deserialize_works_on(name, uuid)
        }
    }
}