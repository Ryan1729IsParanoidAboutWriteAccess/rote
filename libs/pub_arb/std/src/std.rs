/// This crate contains ways to create `proptest` `Strategy` implementations, for types in the rust standard library.

use proptest::prop_compose;

prop_compose!{
    pub fn path_buf()(
        s in ".*\\.fakefile" 
    ) -> std::path::PathBuf {
        std::path::PathBuf::from(s)
    }
}
