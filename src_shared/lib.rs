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

enum LiteCrateVersionStream {
    Full,
    Lite,
    Nightly,
}
impl LiteCrateVersionStream {
    const fn specifies_rust_verison(&self) -> bool {
        matches!(self, Self::Lite | Self::Nightly)
    }
}

const VERSION_STREAM: LiteCrateVersionStream = {
    const VERSION_PATCH: u64 = {
        let patch = env!("CARGO_PKG_VERSION_PATCH");
        if !patch.is_empty() {
            let result = u64::from_str_radix(patch, 10);
            match result {
                Ok(patch) => patch,
                Err(_) => {
                    panic!(
                        "Expecting the version patch part to be a number, but parsing it failed."
                    )
                }
            }
        } else {
            panic!("Patch part of `lite` crate's version is unknown.");
        }
    };

    match VERSION_PATCH % 3 {
        0 => LiteCrateVersionStream::Full,
        1 => LiteCrateVersionStream::Lite,
        2 => LiteCrateVersionStream::Nightly,
        _ => unreachable!(),
    }
};

const _VERIFY_LITE_ON_NIGHTLY_FEATURE_ONLY_FOR_LITE_VERSION: () = {
    #[cfg(feature = "lite_on_nightly")]
    const LITE_ON_NIGHTLY_FEATURE: bool = true;
    #[cfg(not(feature = "lite_on_nightly"))]
    const LITE_ON_NIGHTLY_FEATURE: bool = false;

    assert!(LITE_ON_NIGHTLY_FEATURE == (matches!(VERSION_STREAM, LiteCrateVersionStream::Lite)));
};

const _VERIFY_ONLY_CHOSEN_VERSIONS_SPECIFY_RUST_VERSION: () = {
    const RUST_VERSION_SPECIFICED: bool = {
        // no need to use option_env!(..), because as per
        // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
        // "if one of these values is not provided in the manifest... set to the empty string"
        let rust_version = env!("CARGO_PKG_RUST_VERSION");
        !rust_version.is_empty()
    };

    if RUST_VERSION_SPECIFICED && !VERSION_STREAM.specifies_rust_verison() {
        panic!(
            "RUST_VERSION_SPECIFICED is true, but VERSION_STREAM.specifies_rust_verison() is false"
        )
    }
    if !RUST_VERSION_SPECIFICED && VERSION_STREAM.specifies_rust_verison() {
        panic!(
            "RUST_VERSION_SPECIFICED is false, but VERSION_STREAM.specifies_rust_verison() is true"
        )
    }
    assert!(RUST_VERSION_SPECIFICED == VERSION_STREAM.specifies_rust_verison());
};

const _VERIFY_VERSION_PATCH_MATCHES_VERSION_METADATA: () = {
    const VERSION_METADATA: &str = {
        let pkg_version = env!("CARGO_PKG_VERSION");
        let bytes = pkg_version.as_bytes();

        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'+' {
                break;
            }
            i += 1;
        }
        if i == bytes.len() {
            panic!(
                "No metadata part in the version of `lite` crate (no plus sign in its version)."
            );
        }

        let (_, bytes) = bytes.split_at(i + 1);

        match str::from_utf8(bytes) {
            Ok(slice) => slice,
            Err(_) => panic!(),
        }
    };

    match (VERSION_METADATA.as_bytes(), VERSION_STREAM) {
        (b"FULL", LiteCrateVersionStream::Full) => {}
        (b"LITE", LiteCrateVersionStream::Lite) => {}
        (b"NIGHTLY", LiteCrateVersionStream::Nightly) => {}
        (_, _) => {
            panic!("Patch part of `lite` crate's version doesn't match the version metadata.")
        }
    }
};
