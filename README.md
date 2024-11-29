<div align="center">

<!- TODO: Fix?[![Linebender Zulip, #kurbo stream](https://img.shields.io/badge/Linebender-%23kurbo-red?logo=Zulip)](https://xi.zulipchat.com/#narrow/stream/260979-kurbo) -->
[![dependency status](https://deps.rs/repo/github/linebender/raw_resource_handle/status.svg)](https://deps.rs/repo/github/linebender/raw_resource_handle)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#license)
[![Build status](https://github.com/linebender/raw_resource_handle/workflows/CI/badge.svg)](https://github.com/linebender/raw_resource_handle/actions)
[![Crates.io](https://img.shields.io/crates/v/raw_resource_handle.svg)](https://crates.io/crates/raw_resource_handle)
[![Docs](https://docs.rs/raw_resource_handle/badge.svg)](https://docs.rs/raw_resource_handle)

</div>

The `raw_resource_handle` library provides functionality for storing blobs of data and an associated ID.

## Minimum supported Rust Version (MSRV)

This version of `raw_resource_handle` has been verified to compile with **Rust 1.70** and later.

Future versions of `raw_resource_handle` might increase the Rust version requirement.
It will not be treated as a breaking change and as such can even happen with small patch releases.

<details>
<summary>Click here if compiling fails.</summary>

As time has passed, some of `raw_resource_handle`'s dependencies could have released versions with a higher Rust requirement.
If you encounter a compilation issue due to a dependency and don't want to upgrade your Rust toolchain, then you could downgrade the dependency.

```sh
# Use the problematic dependency's name and version
cargo update -p package_name --precise 0.1.1
```
</details>

## Community

<!-- TODO: Fix [![Linebender Zulip, #kurbo stream](https://img.shields.io/badge/Linebender-%23kurbo-red?logo=Zulip)](https://xi.zulipchat.com/#narrow/stream/260979-kurbo) -->

Discussion of `raw_resource_handle` development happens in the Linebender Zulip at <https://xi.zulipchat.com/>, but there is not yet an established channel.
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
