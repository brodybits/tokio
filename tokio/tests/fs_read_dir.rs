#![warn(rust_2018_idioms)]
#![cfg(all(feature = "full", not(tokio_wasi)))] // Wasi does not support file operations

use tempfile::tempdir;
use tokio::fs;

#[tokio::test]
async fn read_dir_first_entry() {
    let temp_dir = tempdir().unwrap();

    let file_path = temp_dir.path().join("a.txt");

    fs::write(&file_path, b"Hello File!").await.unwrap();

    let mut dir = fs::read_dir(temp_dir.path()).await.unwrap();

    let first_entry = dir.next_entry().await.unwrap().unwrap();

    assert_eq!(first_entry.path(), file_path);
    assert_eq!(first_entry.file_name(), "a.txt");
    assert!(first_entry.metadata().await.unwrap().is_file());
    assert!(first_entry.file_type().await.unwrap().is_file());
}
