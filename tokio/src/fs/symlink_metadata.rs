use crate::fs::asyncify;

use std::fs::Metadata;
use std::io;
use std::path::Path;

//use no_panic::no_panic;

/// Queries the file system metadata for a path.
///
/// This is an async version of [`std::fs::symlink_metadata`][std]
///
/// [std]: fn@std::fs::symlink_metadata
//#[no_panic]
pub async fn symlink_metadata(path: impl AsRef<Path>) -> io::Result<Metadata> {
    let path = path.as_ref().to_owned();
    asyncify(|| std::fs::symlink_metadata(path)).await
}
