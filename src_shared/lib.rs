#![doc = include_str!("../README.md")]
#![no_std]
#![allow(unstable_features)]
#![allow(internal_features)]

#[cfg(feature = "lite_on_nightly")]
use no_link as _;

const _VERIFY_DEFAULT_FEATURE_IS_DISABLED: () = {
    #[cfg(feature = "default_feature")]
    compile_error!("Disable default features for `lite` crate. And there's more. Read the docs!");
};

#[doc(hidden)]
pub mod validate;

// @TODO macros
//
//@TODO in our crate, have: use crate as lite;
//@TODO for export: use ::lite as lite;
use crate as lite;

const VERSION_STREAM: lite::validate::LiteCrateVersionStream = {
    // no need to use option_env!(..), because as per
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
    // "if one of these values is not provided in the manifest... set to the empty string"
    let patch = ::core::env!("CARGO_PKG_VERSION_PATCH");
    lite::validate::parse_version_patch(patch)
};

const _VERIFY_LITE_ON_NIGHTLY_FEATURE_ONLY_FOR_LITE_VERSION: () = {
    #[cfg(feature = "lite_on_nightly")]
    const LITE_ON_NIGHTLY_FEATURE: ::core::primitive::bool = true;
    #[cfg(not(feature = "lite_on_nightly"))]
    const LITE_ON_NIGHTLY_FEATURE: ::core::primitive::bool = false;

    VERSION_STREAM.assert_specifies_feature_lite_on_nightly_exactly_if(LITE_ON_NIGHTLY_FEATURE);
};

const _VERIFY_ONLY_CHOSEN_VERSIONS_SPECIFY_RUST_VERSION: () = {
    const RUST_VERSION_SPECIFICED: ::core::primitive::bool = {
        let rust_version = ::core::env!("CARGO_PKG_RUST_VERSION");
        !rust_version.is_empty()
    };

    VERSION_STREAM.assert_specifies_feature_lite_on_nightly_exactly_if(RUST_VERSION_SPECIFICED);
};

const _VERIFY_VERSION_PATCH_MATCHES_VERSION_METADATA: () = {
    const CARGO_PKG_VERSION: &::core::primitive::str = ::core::env!("CARGO_PKG_VERSION");
    VERSION_STREAM.assert_matches_metadata_part_of_version(CARGO_PKG_VERSION);
};
