<!-- markdownlint-disable MD014 -->

<div align="center">

  <h1><code>near-abi-client-rs</code></h1>

  <p>
    <strong>Library to generate Rust client code with <a href="https://github.com/near/workspaces-rs">workspaces-rs</a> from <a href="https://github.com/near/abi">ABI schemas</a> on NEAR</strong>
  </p>

  <p>
    <a href="https://github.com/near/near-abi-client-rs/actions/workflows/test.yml?query=branch%3Amain"><img src="https://github.com/near/near-abi-client-rs/actions/workflows/test.yml/badge.svg" alt="Github CI Build" /></a>
    <a href="https://crates.io/crates/near-abi-client"><img src="https://img.shields.io/crates/v/near-abi-client.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/near-abi-client"><img src="https://img.shields.io/crates/d/near-abi-client.svg?style=flat-square" alt="Downloads" /></a>
  </p>

</div>

## Release notes

**Release notes and unreleased changes can be found in the [CHANGELOG](CHANGELOG.md)**

## Usage

This crate supports two sets of APIs for users with different needs:
* **Macro-driven**. Gives you a client in a single macro invocation.
* **Generation-based**. Gives you more control and is transparent about what code you end up using, but requires more setup.

### Macro API

Checkout the [`delegator-macro`](https://github.com/near/near-abi-client-rs/tree/main/examples/delegator-macro) example for a standalone project using macro API to get a client and use it.

To generate a struct named `ClientName` based on ABI located at `path/to/abi.json` (relative to the current file's directory):

```rust
mod mymod {
    near_abi_client::generate!(ClientName for "path/to/abi.json");
}
```

Placing the macro invocation inside a `mod` section is optional, but helps reducing unexpected behaviors such as name clashes.

Now, assuming you have a `contract: near_workspaces::Contract` deployed, you can make a call like this:

```rust
let contract = mymod::ClientName { contract };
let res = contract
    .my_method_name(arg1, arg2)
    .await?;
```

### Generation API

Checkout the [`delegator-generation`](https://github.com/near/near-abi-client-rs/tree/main/examples/delegator-generation) example for a standalone project using generation API to generate a client and use it.

First, we need our package to have a `build.rs` file that runs the generation step. The following snippet will generate the client in `abi.rs` under `path/to/out/dir`:

```rust
fn main() -> anyhow::Result<()> {
    near_abi_client::Generator::new("path/to/out/dir".into())
        .file("path/to/abi.json")
        .generate()?;
    Ok(())
}
```

The resulting file, however, is not included in your source set by itself. You have to include it manually; the recommended way is to create a mod with a custom path:

```
#[path = "path/to/out/dir/abi.rs"]
mod mymod;
```

Now, assuming you have a `contract: near_workspaces::Contract` deployed, you can make a call like this:

```rust
let contract = mymod::AbiClient { contract };
let res = contract
    .my_method_name(arg1, arg2)
    .await?;
```

Feel free to explore what other methods `Generator` has to customize the resulting code (e.g. client struct name).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as below, without any additional terms or conditions.

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
