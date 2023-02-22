#![warn(rust_2018_idioms)]
#![cfg(all(feature = "full", not(tokio_wasi)))] // Wasi does not support file operations
#![cfg(windows)]

//use std::io::Write;
//use tempfile::NamedTempFile;
use tokio::fs::OpenOptions;
//use tokio::io::AsyncReadExt;
use windows_sys::Win32::Storage::FileSystem;

#[tokio::test]
#[cfg(windows)]
async fn open_options_custom_flags_windows() {
    // TEST HACK: use Debug output to check the stored data
    assert!(format!(
        "{:?}",
        OpenOptions::new().custom_flags(FileSystem::FILE_FLAG_DELETE_ON_CLOSE)
    )
    .contains("custom_flags: 67108864,"));
}
