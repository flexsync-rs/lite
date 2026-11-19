#![allow(unused_imports)]

#[allow(dead_code)]
struct Dummy;

use Dummy as bool;
use Dummy as core;
use Dummy as lite;
use Dummy as std;

const _: () = {
    // Surprisingly, as of Rust 1.100.0, the following does even evaluate (without a panic) when
    // (symlinked from) under tests/ directory. However, unsure if this is guaranteed, hence _not_
    // making any fallable values a `const`.
    if false {
        ::lite::internal_check_my_crate_manifest!(
            lite_on_nightly_feature,
            rust_version_only_for_lite_or_nightly
        );
        ::lite::internal_check_my_crate_manifest!(lite_on_nightly_feature);
        ::lite::internal_check_my_crate_manifest!(rust_version_only_for_lite_or_nightly);
        ::lite::internal_check_my_crate_manifest!();
    };
};
