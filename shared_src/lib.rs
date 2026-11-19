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

#[macro_export]
macro_rules! check_my_crate_manifest {
    ($( $param:tt ),*) => {
        const _: () = {
            $crate::internal_check_my_crate_manifest!( $( $param ),* )
        };
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! internal_check_my_crate_manifest {
    (
        $( $param:tt ),*
    ) => {
        {
            // The other branches of this macro _would_ be much easier if version_stream were a
            // constant instead. However, we make it a variable, and we pass it around, so that
            // shared_tests/ensure_full_paths_in_macros.rs can compile this without evaluating it.
            let version_stream = {
                // no need to use option_env!(..), because as per
                // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
                // "if one of these values is not provided in the manifest... set to the empty
                // string"
                let patch = ::core::env!("CARGO_PKG_VERSION_PATCH");
                $crate::validate::parse_version_patch(patch)
            };
            {
                const CARGO_PKG_VERSION: &::core::primitive::str =
                    ::core::env!("CARGO_PKG_VERSION");
                version_stream.assert_matches_metadata_part_of_version(CARGO_PKG_VERSION);
            }

            $crate::internal_check_my_crate_manifest!(@param version_stream, $( $param ),*);
        }
    };

    (@param $version_stream:expr, lite_on_nightly_feature $(, $param:tt )* ) => {{
        #[cfg(feature = "lite_on_nightly")]
        const LITE_ON_NIGHTLY_FEATURE: ::core::primitive::bool = true;
        #[cfg(not(feature = "lite_on_nightly"))]
        const LITE_ON_NIGHTLY_FEATURE: ::core::primitive::bool = false;

        $version_stream.assert_specifies_feature_lite_on_nightly_exactly_if(LITE_ON_NIGHTLY_FEATURE);

        $crate::internal_check_my_crate_manifest!(@param $version_stream, $( $param ),*);
    }};

    (@param $version_stream:expr, rust_version_only_for_lite_or_nightly $(, $param:tt )* ) => {{
        let rust_version_specified: ::core::primitive::bool = {
            let rust_version = ::core::env!("CARGO_PKG_RUST_VERSION");
            !rust_version.is_empty()
        };

        $version_stream.assert_specifies_rust_version_exactly_if(rust_version_specified);

        $crate::internal_check_my_crate_manifest!(@param $version_stream, $( $param ),*);
    }};

    (@param $version_stream:expr,) => {};
}

check_my_crate_manifest!(
    lite_on_nightly_feature,
    rust_version_only_for_lite_or_nightly
);
// The order doesn't matter
#[cfg(debug_assertions)]
check_my_crate_manifest!(
    rust_version_only_for_lite_or_nightly,
    lite_on_nightly_feature
);
#[cfg(debug_assertions)]
check_my_crate_manifest!(rust_version_only_for_lite_or_nightly);
#[cfg(debug_assertions)]
check_my_crate_manifest!(lite_on_nightly_feature);
#[cfg(debug_assertions)]
check_my_crate_manifest!();
