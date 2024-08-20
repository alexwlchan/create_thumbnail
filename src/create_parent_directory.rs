use std::fs;
use std::io::Result;
use std::path::PathBuf;

/// Create the parent directory of a given path.
///
/// Example:
///
///     create_parent_directory("path/to/images/index.html")
///      ~> creates "path/to/images/"
///
pub fn create_parent_directory(path: &PathBuf) -> Result<()> {
    // Quoting from the Rust docs for PathBuf.parent() [1]:
    //
    //     Returns None if the path terminates in a root or prefix,
    //     or if it’s the empty string.
    //
    // This function should only ever be called on paths to files, so
    // .parent() will never return None.
    //
    // [1]: https://doc.rust-lang.org/std/path/struct.PathBuf.html#method.parent
    let parent_dir = path.parent().unwrap();

    fs::create_dir_all(&parent_dir)
}

#[cfg(test)]
mod test_create_parent_directory {
    use super::*;
    use crate::test_utils::test_dir;

    #[test]
    fn it_creates_a_directory() {
        let t = test_dir();
        assert!(!t.exists());

        let path = t.join("path/to/images").join("index.html");
        assert!(create_parent_directory(&path).is_ok());

        assert!(t.exists());
        assert!(t.join("path/to/images").exists());
        assert!(!path.exists());
    }
}
