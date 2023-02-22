#![warn(rust_2018_idioms)]
#![cfg(all(feature = "full", not(tokio_wasi)))] // Wasi does not support file operations

use std::io::Write;
use tempfile::NamedTempFile;
use tokio::fs::OpenOptions;
use tokio::io::AsyncReadExt;

const HELLO: &[u8] = b"hello world...";

#[tokio::test]
async fn open_with_open_options_and_read() {
    let mut tempfile = NamedTempFile::new().unwrap();
    tempfile.write_all(HELLO).unwrap();

    let mut file = OpenOptions::new()
        .read(true)
        .open(tempfile)
        .await
        .unwrap();

    let mut buf = [0; 1024];
    let n = file.read(&mut buf).await.unwrap();

    assert_eq!(n, HELLO.len());
    assert_eq!(&buf[..n], HELLO);
}

#[tokio::test]
async fn open_options_write() {
    assert!(format!("{:?}", OpenOptions::new().write(true)).contains("write: true"));
}

#[tokio::test]
async fn open_options_append() {
    assert!(format!("{:?}", OpenOptions::new().append(true)).contains("append: true"));
}

#[tokio::test]
async fn open_options_truncate() {
    assert!(format!("{:?}", OpenOptions::new().truncate(true)).contains("truncate: true"));
}

#[tokio::test]
async fn open_options_create() {
    assert!(format!("{:?}", OpenOptions::new().create(true)).contains("create: true"));
}

#[tokio::test]
async fn open_options_create_new() {
    assert!(format!("{:?}", OpenOptions::new().create_new(true)).contains("create_new: true"));
}

#[tokio::test]
async fn open_options_mode() {
    assert!(format!("{:?}", OpenOptions::new().mode(0o644)).contains("mode: 420"));
}

#[tokio::test]
async fn open_options_custom_flags() {
    assert!(format!("{:?}", OpenOptions::new().custom_flags(libc::O_TRUNC)).contains("custom_flags: 1024"));
}
