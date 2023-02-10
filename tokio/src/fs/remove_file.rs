use crate::fs::asyncify;

use std::io;
use std::path::Path;

use no_panic::no_panic;

/// Removes a file from the filesystem.
///
/// Note that there is no guarantee that the file is immediately deleted (e.g.
/// depending on platform, other open file descriptors may prevent immediate
/// removal).
///
/// This is an async version of [`std::fs::remove_file`][std]
///
/// [std]: std::fs::remove_file
#[no_panic]
pub async fn remove_file(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(move || std::fs::remove_file(path)).await
}
