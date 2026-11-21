#![no_std]
#![no_implicit_prelude]
#![allow(unused_imports)]

// even with no implicit prelude, there are some macros and basic types:
// - https://doc.rust-lang.org/reference/names/preludes.html#r-names.preludes.no_implicit_prelude.implicitly-imported-macros
// - https://doc.rust-lang.org/reference/names/preludes.html#language-prelude
//
// Hence we define ours, so as to test that the macros always use full paths.
mod dummy_mod {}

#[allow(unused_macros)]
macro_rules! dummy_macro {
    () => {};
}

#[allow(dead_code)]
struct DummyType;

use dummy_macro as env;
use dummy_mod as core;
use dummy_mod as lite;
use dummy_mod as std;
use DummyType as bool;
use DummyType as str;

::lite::check_my_crate_manifest!(
    specifies_lite_on_nightly_feature,
    specifies_rust_version_only_for_lite_or_nightly
);
::lite::check_my_crate_manifest!(
    specifies_rust_version_only_for_lite_or_nightly,
    specifies_lite_on_nightly_feature
);
::lite::check_my_crate_manifest!(specifies_lite_on_nightly_feature);
::lite::check_my_crate_manifest!(specifies_rust_version_only_for_lite_or_nightly);
::lite::check_my_crate_manifest!();
