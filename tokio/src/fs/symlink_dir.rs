use crate::fs::asyncify;

use std::io;
use std::path::Path;

use no_panic::no_panic;

/// Creates a new directory symlink on the filesystem.
///
/// The `dst` path will be a directory symbolic link pointing to the `src`
/// path.
///
/// This is an async version of [`std::os::windows::fs::symlink_dir`][std]
///
/// [std]: std::os::windows::fs::symlink_dir
#[no_panic]
pub async fn symlink_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    let src = src.as_ref().to_owned();
    let dst = dst.as_ref().to_owned();

    asyncify(move || std::os::windows::fs::symlink_dir(src, dst)).await
}
