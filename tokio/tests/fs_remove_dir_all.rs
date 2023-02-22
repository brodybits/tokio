#![warn(rust_2018_idioms)]
#![cfg(all(feature = "full", not(tokio_wasi)))] // Wasi does not support file operations

use tempfile::tempdir;
use tokio::fs;

#[tokio::test]
async fn remove_dir_all() {
    let temp_dir = tempdir().unwrap();

    let file_path = temp_dir.path().join("a.txt");

    fs::write(&file_path, b"Hello File!").await.unwrap();

    fs::remove_dir_all(temp_dir.path()).await.unwrap();
}
