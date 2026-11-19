#![allow(unused_imports)]

#[allow(dead_code)]
struct Dummy;

use Dummy as bool;
use Dummy as core;
use Dummy as lite;
use Dummy as std;

const _: () = {
    ::lite::internal_check_my_crate_manifest!(
        lite_on_nightly_feature,
        rust_version_only_for_lite_or_nightly
    );
    ::lite::internal_check_my_crate_manifest!(lite_on_nightly_feature);
    ::lite::internal_check_my_crate_manifest!(rust_version_only_for_lite_or_nightly);
    ::lite::internal_check_my_crate_manifest!();
};
