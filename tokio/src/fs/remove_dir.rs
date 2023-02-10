use crate::fs::asyncify;

use std::io;
use std::path::Path;

//use no_panic::no_panic;

/// Removes an existing, empty directory.
///
/// This is an async version of [`std::fs::remove_dir`](std::fs::remove_dir)
//#[no_panic]
pub async fn remove_dir(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(move || std::fs::remove_dir(path)).await
}
