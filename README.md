<div align="center">

# Linebender Resource Handle

**Shared handle for immutable blobs with dynamic lifetime**

[![Latest published version.](https://img.shields.io/crates/v/linebender_resource_handle.svg)](https://crates.io/crates/linebender_resource_handle)
[![Documentation build status.](https://img.shields.io/docsrs/linebender_resource_handle.svg)](https://docs.rs/linebender_resource_handle)
[![Apache 2.0 or MIT license.](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](#license)
\
[![Linebender Zulip, #Linebender Resource Handle topic.](https://img.shields.io/badge/Linebender-%23rust--ui-blue?logo=Zulip)](https://xi.zulipchat.com/#narrow/channel/422907-rust-ui/topic/Linebender.20Resource.20Handle/with/538254686)
[![GitHub Actions CI status.](https://img.shields.io/github/actions/workflow/status/linebender/raw_resource_handle/ci.yml?logo=github&label=CI)](https://github.com/linebender/raw_resource_handle/actions)
[![Dependency staleness status.](https://deps.rs/repo/github/linebender/raw_resource_handle/status.svg)](https://deps.rs/repo/github/linebender/raw_resource_handle)

</div>

<!-- We use cargo-rdme to update the README with the contents of lib.rs.
To edit the following section, update it in lib.rs, then run:
cargo rdme --workspace-project=masonry_core
Full documentation at https://github.com/orium/cargo-rdme -->

<!-- Intra-doc links used in lib.rs should be evaluated here.
See https://linebender.org/blog/doc-include/ for related discussion. -->
[`FontData`]: https://docs.rs/linebender_resource_handle/latest/linebender_resource_handle/struct.FontData.html

<!-- markdownlint-disable MD053 -->
<!-- cargo-rdme start -->

Linebender Resource Handle provides functionality for storing blobs of data and an associated ID.
This primitive is adapted in Peniko to store images, but the [`FontData`] type lives in this crate.
This crate is designed to allow making semver incompatible releases of Parley and Vello, whilst allowing them to be cross-compatible.

This crate is not intended for long-term use, and we expect our resource handling story to change.
That's the reason that this crate has the organisation name ("Linebender") in its crate name; we avoid squatting a more general name after we discontinue it.

## Features

The following crate [feature flags](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features) are available:

- `std` (enabled by default): Enable future features which require the standard library.
  This feature is provided for forwards compatibility only, and current behaviour is the same whether or not it is enabled.
- `serde`: Implement [`serde::Serialize`] and [`serde::Deserialize`] for the types in the crate.
- `stable_deref_trait_v1`: Implements the [`StableDeref`](stable_deref_trait::StableDeref) trait for `Blob`, which can be
  used with crates like `yoke` to enable zero-copy deserialization of data stored in the `Blob`.

<!-- cargo-rdme end -->
<!-- markdownlint-enable MD053 -->

## Minimum supported Rust Version (MSRV)

This version of Linebender Resource Handle has been verified to compile with **Rust 1.70** and later.

Future versions of Linebender Resource Handle might increase the Rust version requirement.
It will not be treated as a breaking change and as such can even happen with small patch releases.

## Community

[![Linebender Zulip, #Linebender Resource Handle topic.](https://img.shields.io/badge/Linebender-%23rust--ui-blue?logo=Zulip)](https://xi.zulipchat.com/#narrow/channel/422907-rust-ui/topic/Linebender.20Resource.20Handle/with/538254686)

Discussion of Linebender Resource Handle development happens in the [Linebender Zulip](https://xi.zulipchat.com/), specifically in [#rust ui > Linebender Resource Handle](https://xi.zulipchat.com/#narrow/channel/422907-rust-ui/topic/Linebender.20Resource.20Handle/with/538254686).
All public content can be read without logging in

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Contributions are welcome by pull request. The [Rust code of conduct] applies.
Please feel free to add your name to the [AUTHORS] file in any substantive pull request.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

[Rust Code of Conduct]: https://www.rust-lang.org/policies/code-of-conduct
[AUTHORS]: ./AUTHORS
