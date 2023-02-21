#![warn(rust_2018_idioms)]
#![cfg(all(feature = "full", not(tokio_wasi)))] // Wasi does not support file operations

use tempfile::tempdir;
use tokio::fs;

#[tokio::test]
async fn read_dir_first_entry() {
    let dir = tempdir().unwrap();

    let file_path = dir.path().join("a.txt");

    fs::write(&file_path, b"Hello File!").await.unwrap();

    let mut dir = fs::read_dir(dir.path()).await.unwrap();

    let first_entry = dir.next_entry().await.unwrap();

    assert_eq!(first_entry.unwrap().file_name(), "a.txt");
}
