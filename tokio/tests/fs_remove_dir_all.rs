#![warn(rust_2018_idioms)]
#![cfg(all(feature = "full", not(tokio_wasi)))] // Wasi does not support fs operations

use tempfile::tempdir;
use tokio::fs;

#[tokio::test]
async fn remove_dir_all() {
    let temp_dir = tempdir().unwrap();

    let test_dir = temp_dir.path().join("test");

    let file_path = test_dir.path().join("a.txt");

    fs::write(&file_path, b"Hello File!").await.unwrap();

    fs::remove_dir_all(test_dir.path()).await.unwrap();

    // test dir should no longer exist
    match fs::try_exists(test_dir).await {
        Ok(exists) => assert!(!exists),
        Err(info) => println!("ignoring error after remove_dir_all, see info: {:?}", info),
    };

    // contents should no longer exist
    match fs::try_exists(file_path).await {
        Ok(exists) => assert!(!exists),
        Err(info) => println!("ignoring error after remove_dir_all, see info: {:?}", info),
    };
}
