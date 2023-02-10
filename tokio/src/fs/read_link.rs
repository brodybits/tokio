use crate::fs::asyncify;

use std::io;
use std::path::{Path, PathBuf};

//use no_panic::no_panic;

/// Reads a symbolic link, returning the file that the link points to.
///
/// This is an async version of [`std::fs::read_link`][std]
///
/// [std]: std::fs::read_link
//#[no_panic]
pub async fn read_link(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref().to_owned();
    asyncify(move || std::fs::read_link(path)).await
}
