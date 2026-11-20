#![doc = include_str!("../../README.md")]
#![no_std]
// The following attributes enforce `nightly`.
//
// This serves to _intentionally_ make _old_ `+LITE` and `+NIGHTLY` versions of `lite` crate
// _automatically_ _fail_ on `stable` Rust, if they (such old `+LITE` or `+NIGHTLY` versions of
// `lite` crate) were _accidentally_ left unyanked long enough so that their required Rust version
// became `stable`.
#![allow(unstable_features)]
#![allow(internal_features)]
#![allow(unused_features)]
#![feature(sized_hierarchy)] // https://github.com/rust-lang/rust/issues/144404
#![feature(const_destruct)] // https://github.com/rust-lang/rust/issues/133214
#![feature(unsafe_unpin)] // https://github.com/rust-lang/rust/issues/125735

pub mod central;
