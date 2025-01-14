#![allow(unknown_lints, unexpected_cfgs)]
#![allow(clippy::needless_doctest_main)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]
#![cfg_attr(docsrs, feature(doc_cfg))]

#![no_std]

//! Utilities for working with Tokio.
//!
//! This crate is not versioned in lockstep with the core
//! [`tokio`] crate. However, `tokio-util` _will_ respect Rust's
//! semantic versioning policy, especially with regard to breaking changes.
//!
//! [`tokio`]: https://docs.rs/tokio

#[macro_use]
mod cfg;

#[cfg(feature = "std")]
mod loom;

cfg_codec! {
    #[macro_use]
    mod tracing;

    pub mod codec;
}

cfg_net! {
    #[cfg(not(target_arch = "wasm32"))]
    pub mod udp;
    pub mod net;
}

cfg_compat! {
    pub mod compat;
}

cfg_io! {
    pub mod io;
}

cfg_rt! {
    pub mod context;
    pub mod task;
}

cfg_time! {
    pub mod time;
}

#[cfg(feature = "std")]
pub mod sync;

#[cfg(feature = "std")]
pub mod either;

pub use bytes;

mod util;

extern crate alloc;

pub(crate) mod alias {
    pub(crate) mod std {
        pub(crate) mod prelude {
            pub(crate) use super::boxed::Box;
            pub(crate) use super::string::String;
            pub(crate) use super::vec;
            pub(crate) use super::vec::Vec;
        }

        pub(crate) use core::{cell, fmt, future, mem, pin, ptr, task};

        pub(crate) use crate::alloc::{borrow, boxed, string, vec};

        #[cfg(feature = "std")]
        extern crate std;

        #[cfg(feature = "std")]
        pub(crate) use std::io;

        #[cfg(feature = "std")]
        pub(crate) use std::{alloc, error, collections, hash, panic, result, sync, thread_local};
    }
}
