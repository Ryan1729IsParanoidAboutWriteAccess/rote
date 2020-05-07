/// This module was originally created to make sure every change to the current index went 
/// through a single path so we could more easily track down a bug where the index was 
/// improperly set.
use editor_types::{Cursor};
use g_i::SelectableVec1;
use macros::{d, u};
use platform_types::*;
use parsers::{ParserKind};
use text_buffer::{TextBuffer};
use search::{SearchResults};
use panic_safe_rope::{RopeSlice, RopeSliceTrait};

use std::path::PathBuf;

#[derive(Clone, Debug, Default)]
pub struct EditorBuffer {
    pub text_buffer: TextBuffer,
    pub name: BufferName,
    //TODO: Set `current_range` to something as close as possible to being on screen of haystack
    // whenever this changes
    pub search_results: SearchResults,
    // If this is none, then it was not set by the user, and
    // we will use the default.
    parser_kind: Option<ParserKind>,
}

impl EditorBuffer {
    fn rope_hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.text_buffer.rope_hash(state);
    }

    fn non_rope_hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.text_buffer.non_rope_hash(state);
    }
}

impl From<&EditorBuffer> for String {
    fn from(e_b: &EditorBuffer) -> Self {
        (&e_b.text_buffer).into()
    }
}

impl From<&mut EditorBuffer> for String {
    fn from(e_b: &mut EditorBuffer) -> Self {
        (&e_b.text_buffer).into()
    }
}

impl EditorBuffer {
    pub fn new<I: Into<TextBuffer>>(name: BufferName, s: I) -> Self {
        Self {
            name,
            ..d!()
        }
    }

    pub fn get_parser_kind(&self) -> ParserKind {
        u!{ParserKind}
        self.parser_kind.unwrap_or_else(|| {
            match self.name.get_extension_or_empty() {
                "rs" => Rust(d!()),
                _ => Plaintext,
            }
        })
    }

    pub fn next_language(&mut self) {
        self.parser_kind = Some(
            self.get_parser_kind().next().unwrap_or_default()
        );
    }

    pub fn advance_or_refresh_search_results(&mut self, needle: RopeSlice) {
        if needle == self.search_results.needle {
            self.advance_to_next_search_result(needle);
        } else {
            dbg!("advance_or_refresh_search_results");
            self.refresh_search_results(needle);
            self.advance_to_next_search_result(needle);
        }
    }

    fn advance_to_next_search_result(&mut self, needle: RopeSlice) {
        dbg!(needle.len_bytes());
        if needle.len_bytes() > 0 {
            let search_results = &mut self.search_results;
            let len = search_results.ranges.len();
            search_results.current_range += 1;
            if search_results.current_range >= len {
                search_results.current_range = 0;
            }

            if let Some(pair) = self
                .search_results
                .ranges
                .get(self.search_results.current_range)
            {
                let c: Cursor = pair.into();
                self
                    .set_cursor(c, ReplaceOrAdd::Replace);            
            }
        }
    }

    pub fn refresh_search_results(&mut self, needle: RopeSlice) {        
        self.search_results.refresh(
            needle,
            self.text_buffer.borrow_rope()
        );
    }
}

/// The collection of files opened for editing, and/or in-memory scratch buffers.
/// Guaranteed to have at least one buffer in it at all times.
#[derive(Clone, Debug, Default)]
pub struct EditorBuffers {
    buffers: SelectableVec1<EditorBuffer>,
    last_non_rope_hash: u64,
    last_full_hash: Option<u64>,
}

impl EditorBuffers {
    #[perf_viz::record]
    pub fn should_render_buffer_views(&mut self) -> bool {
        true
    }

    #[perf_viz::record]
    fn rope_hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for b in self.buffers.iter() {
            b.rope_hash(state);
        }
    }

    #[perf_viz::record]
    fn non_rope_hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for b in self.buffers.iter() {
            b.non_rope_hash(state);
        }
    }
}

impl EditorBuffers {
    pub fn new(buffer: EditorBuffer) -> Self {
        Self {
            buffers: SelectableVec1::new(buffer),
            ..d!()
        }
    }

    /// Since there is always at least one buffer, this always returns at least 1.
    pub fn len(&self) -> g_i::Length {
        self.buffers.len()
    }

    /// The index of the currectly selected buffer.
    pub fn current_index(&self) -> g_i::Index {
        self.buffers.current_index()
    }

    pub fn current_index_part(&self) -> g_i::IndexPart {
        self.buffers.current_index_part()
    }

    pub fn set_current_index(&mut self, index: g_i::Index) -> bool {
        self.buffers.set_current_index(index)
    }

    pub fn get_current_buffer(&self) -> &EditorBuffer {
        self.buffers.get_current_element()
    }

    pub fn get_current_buffer_mut(&mut self) -> &mut EditorBuffer {
        self.buffers.get_current_element_mut()
    }

    pub fn append_index(&self) -> g_i::Index {
        self.buffers.append_index()
    }

    pub fn push_and_select_new(&mut self, buffer: EditorBuffer) {
        self.buffers.push_and_select_new(buffer);
    }

    pub fn index_with_name(&self, name: &BufferName) -> Option<g_i::Index> {
        let mut index = None;
        for (i, buffer) in self.buffers.iter_with_indexes() {
            if buffer.name == *name {
                index = Some(i);
                break;
            }
        }
        index
    }

    pub fn add_or_select_buffer(&mut self, name: BufferName, str: String) {
        if let Some(index) = self.index_with_name(&name) {
            self.set_current_index(index);

            if name == d!() && usize::from(self.buffers.len()) <= 1 {
                let buffer = &mut self.get_current_buffer_mut().text_buffer;
                if buffer.has_no_edits() {
                    *buffer = str.into();
                }
            }
        } else {
            self.buffers.push_and_select_new(EditorBuffer::new(name, str));
        };
    }

    /// Returns `Some` iff a path was actually set.
    pub fn set_path(&mut self, index: g_i::Index, path: PathBuf) -> Option<()> {
        if let Some(b) = self.buffers.get_mut(index) {
            (*b).name = BufferName::Path(path);
            Some(())
        } else {
            None
        }
    }

    pub fn adjust_selection(&mut self, adjustment: SelectionAdjustment) {
        self.buffers.adjust_selection(adjustment);
    }

    pub fn close_buffer(&mut self, index: g_i::Index) {
        self.buffers.remove_if_present(index);
    }

    pub fn buffers(&self) -> &SelectableVec1<EditorBuffer> {
        &self.buffers
    }
}

impl EditorBuffers {
    pub fn iter(&self) -> std::slice::Iter<EditorBuffer> {
        self.buffers.iter()
    }

    pub fn iter_with_indexes(&self) -> g_i::IterWithIndexes<EditorBuffer> {
        self.buffers.iter_with_indexes()
    }
}