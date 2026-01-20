// Copyright 2024 the Raw Resource Handle Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// After you edit the crate's doc comment, run this command, then check README.md for any missing links
// cargo rdme --workspace-project=linebender_resource_handle

//! Linebender Resource Handle provides functionality for storing blobs of data and an associated ID.
//! This primitive is adapted in Peniko to store images, but the [`FontData`] type lives in this crate.
//! This crate is designed to allow making semver incompatible releases of Parley and Vello, whilst allowing them to be cross-compatible.
//!
//! This crate is not intended for long-term use, and we expect our resource handling story to change.
//! That's the reason that this crate has the organisation name ("Linebender") in its crate name; we avoid squatting a more general name after we discontinue it.
//!
//! # Features
//!
//! The following crate [feature flags](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features) are available:
//!
//! - `std` (enabled by default): Enable future features which require the standard library.
//!   This feature is provided for forwards compatibility only, and current behaviour is the same whether or not it is enabled.
//! - `serde`: Implement [`serde::Serialize`] and [`serde::Deserialize`] for the types in the crate.
//! - `stable_deref_trait_v1`: Implements the [`StableDeref`](stable_deref_trait::StableDeref) trait for `Blob`, which can be
//!   used with crates like `yoke` to enable zero-copy deserialization of data stored in the `Blob`.

// LINEBENDER LINT SET - lib.rs - v3
// See https://linebender.org/wiki/canonical-lints/
// These lints shouldn't apply to examples or tests.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
// These lints shouldn't apply to examples.
#![warn(clippy::print_stdout, clippy::print_stderr)]
// Targeting e.g. 32-bit means structs containing usize can give false positives for 64-bit.
#![cfg_attr(target_pointer_width = "64", warn(clippy::trivially_copy_pass_by_ref))]
// END LINEBENDER LINT SET
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

#[cfg(feature = "std")]
// Ensure that we don't compile if you're using the std feature on a platform without `std`
extern crate std as _;

mod blob;
mod font;

pub use blob::{Blob, BlobStorage, WeakBlob};
pub use font::FontData;

#[cfg(test)]
mod tests {
    // CI will fail unless cargo nextest can execute at least one test per workspace.
    // Delete this dummy test once we have an actual real test.
    #[test]
    fn dummy_test_until_we_have_a_real_test() {}
}
