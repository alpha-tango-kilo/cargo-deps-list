# cargo-deps-list

[![Codeberg CI](https://ci.codeberg.org/api/badges/alpha-tango-kilo/cargo-deps-list/status.svg)](https://ci.codeberg.org/alpha-tango-kilo/cargo-deps-list)
[![Crates.io](https://img.shields.io/crates/v/cargo-deps-list.svg)](https://crates.io/crates/cargo-deps-list)

The quick-and-dirty successor to [cargo-real-deps](https://lib.rs/cargo-real-deps)

Same features, but way simpler, and compiles fast

Born out of my annoyance that cargo doesn't have a way to give you a straight answer about the number of dependencies your project has.
Now, it does

## What it does

It processes the output of `cargo tree` to get a list of all the unique dependencies, and gives you a count of them.
It is not a sophisticated program lol

## Installation

Install using Cargo:

```shell
cargo install cargo-deps-list
```

## Usage

Refer to `cargo tree --help`, as all `cargo-deps-list` does is pass your arguments to `cargo tree`.
Don't provide `--prefix` though, as that's set by `cargo-deps-list`

Recommended use cases:
* `cargo deps-list` - all the dependencies used for everything
* `cargo deps-list --edges normal` - dependencies used in your final binary
* `cargo deps-list --manifest-path=PATH` - for checking the dependencies of a project not in your current working directory. `PATH` must point to a `Cargo.toml` file
