#![warn(rust_2018_idioms)]
#![cfg(all(feature = "full", not(tokio_wasi)))] // Wasi does not support file operations

use tempfile::tempdir;
use tokio::fs;

#[tokio::test]
async fn rename_file() {
    let temp_dir = tempdir().unwrap();

    let file_path = temp_dir.path().join("a.txt");

    fs::write(&file_path, b"Hello File!").await.unwrap();

    assert!(fs::try_exists(&file_path).await.unwrap());

    let new_file_path = temp_dir.path().join("b.txt");

    fs::rename(&file_path, &new_file_path).await.unwrap();

    assert!(fs::try_exists(new_file_path).await.unwrap());

    // original file should no longer exist
    match fs::try_exists(file_path).await {
        Ok(exists) => assert_eq!(exists, false),
        Err(info) => println!("ignoring error after remove, see info: {:?}", info),
    };
}
