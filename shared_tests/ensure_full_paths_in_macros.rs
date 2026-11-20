#![allow(unused_imports)]

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

const _: () = {
    ::lite::internal_check_my_crate_manifest!(
        lite_on_nightly_feature,
        rust_version_only_for_lite_or_nightly
    );
    ::lite::internal_check_my_crate_manifest!(lite_on_nightly_feature);
    ::lite::internal_check_my_crate_manifest!(rust_version_only_for_lite_or_nightly);
    ::lite::internal_check_my_crate_manifest!();
};
