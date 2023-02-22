#![warn(rust_2018_idioms)]
#![cfg(all(feature = "full", not(tokio_wasi)))] // Wasi does not support directory operations

use tokio::fs;

#[tokio::test]
#[cfg(unix)]
async fn canonicalize_dir() {
    assert_eq!(
        fs::canonicalize("/.")
            .await
            .unwrap()
            .to_str()
            .unwrap(),
        "/"
    );
}

#[tokio::test]
#[cfg(windows)]
async fn canonicalize_dir() {
    // 2-step let bindings due to Rust memory semantics
    let dir_path = fs::canonicalize("C:\\.\\")
        .await
        .unwrap();

    let dir_name = dir_path.to_str().unwrap();

    assert!(dir_name.starts_with("\\\\"));
    assert!(dir_name.ends_with("C:\\"));
}
