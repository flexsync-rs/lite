//! _NOT_ a part of public API.

const fn parse_version_metadata(pkg_version: &'static str) -> &'static str {
    // cargo requires metadata part to be alphanumeric only, so we can handle it as ASCII
    assert!(pkg_version.is_ascii());
    let bytes = pkg_version.as_bytes();

    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'+' {
            break;
        }
        i += 1;
    }
    if i == bytes.len() {
        panic!("No metadata part in the crate's version (no plus sign in its version).");
    }

    let (_, bytes) = bytes.split_at(i + 1);

    match str::from_utf8(bytes) {
        Ok(slice) => slice,
        Err(_) => panic!("Internal error when parsing metadata part of the crate's version."),
    }
}

/// _NOT_ a part of public API.
#[doc(hidden)]
pub enum LiteCrateVersionStream {
    Full,
    Lite,
    Nightly,
}
impl LiteCrateVersionStream {
    pub const fn specifies_feature_lite_on_nightly(&self) -> bool {
        matches!(self, Self::Lite)
    }
    pub const fn assert_specifies_feature_lite_on_nightly_exactly_if(&self, condition: bool) {
        assert!(self.specifies_feature_lite_on_nightly() == condition);
    }

    pub const fn specifies_rust_verison(&self) -> bool {
        matches!(self, Self::Lite | Self::Nightly)
    }
    pub const fn assert_specifies_rust_verison_exactly_if(&self, condition: bool) {
        if self.specifies_rust_verison() && !condition {
            panic!("self.specifies_rust_verison() is true, but condition is false");
        }
        if !self.specifies_rust_verison() && condition {
            panic!("self.specifies_rust_verison() is false, but condition is true");
        }
        // redundant, but redundancy is good
        assert!(self.specifies_rust_verison() == condition);
    }

    pub const fn assert_matches_metadata_part_of_version(&self, pkg_version: &'static str) {
        let metadata = parse_version_metadata(pkg_version);

        match (metadata.as_bytes(), self) {
            (b"FULL", LiteCrateVersionStream::Full) => {}
            (b"LITE", LiteCrateVersionStream::Lite) => {}
            (b"NIGHTLY", LiteCrateVersionStream::Nightly) => {}
            (_, _) => {
                panic!("Patch part of the crate's version doesn't match the version metadata.")
            }
        }
    }
}

/// _NOT_ a part of public API.
pub const fn parse_version_patch(patch: &str) -> LiteCrateVersionStream {
    let numeric = if !patch.is_empty() {
        let result = u64::from_str_radix(patch, 10);
        match result {
            Ok(patch) => patch,
            Err(_) => {
                panic!("Expecting the version patch part to be a number, but parsing it failed.")
            }
        }
    } else {
        panic!("Patch part of the crate's version is unknown.");
    };

    match numeric % 3 {
        0 => LiteCrateVersionStream::Full,
        1 => LiteCrateVersionStream::Lite,
        2 => LiteCrateVersionStream::Nightly,
        _ => unreachable!(),
    }
}
