# Contributor Guide

## Prepare the development environment

### Install the Rust toolchain

Install the [Rust](https://www.rust-lang.org/) toolchain
by installing [Rustup](https://www.rust-lang.org/tools/install)
with the standard and
the [nightly](https://rust-lang.github.io/rustup/concepts/channels.html#working-with-nightly-rust)
toolchains.

### Setup the Rust toolchain

If you are using a shell capable of running
[Bash](https://www.gnu.org/software/bash/) scripts,
run the following from the project root directory:

```shell
$ ./setup.sh
```

Otherwise, follow these manual steps:

1. Install [Rustfmt](https://github.com/rust-lang/rustfmt).
2. Install [Clippy](https://github.com/rust-lang/rust-clippy).
3. Install the [cargo-deny](https://crates.io/crates/cargo-deny) Cargo plugin.
4. Install the [cargo-udeps](https://crates.io/crates/cargo-udeps) Cargo plugin.
5. If an installation of the pugins cargo-deny, cargo-udeps fails,
   you may need to install
   [pkg-config](https://www.freedesktop.org/wiki/Software/pkg-config/),
   [libssl-dev](https://www.openssl.org/). For example:

   `$ sudo apt-get install pkg-config && sudo apt-get install libssl-dev`

## Build-related commands

This project uses [Cargo](https://doc.rust-lang.org/cargo/index.html) for build automation.

Run from the project root directory:

| #   | Command                                                                                                                                                                                             | Description                                                                                                                                                                              |
|-----|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0   | `cargo deny check && cargo +nightly udeps && cargo +nightly clippy --all-targets --all-features && cargo +nightly fmt --check && cargo +nightly test && cargo +nightly doc --no-deps -Zrustdoc-map` | Analyzes and reports errors, checks style, runs tests.                                                                                                                                   |
| 0.1 | `cargo +nightly fmt`                                                                                                                                                                                | Reformats the code using`rustfmt`.                                                                                                                                                       |
| 0.2 | `cargo clean`                                                                                                                                                                                       | Deletes the`target` directory.                                                                                                                                                           |
| 1   | `cargo +nightly doc --no-deps -Zrustdoc-map --open`                                                                                                                                                 | Generates documentation and opens it in a browser. See <https://doc.rust-lang.org/cargo/reference/unstable.html#rustdoc-map> .                                                           |
| 1.1 | `cargo clean --doc`                                                                                                                                                                                 | Deletes the `target/doc` directory.                                                                                                                                                      |
| 2   | `cargo +nightly build`                                                                                                                                                                              | Builds the project into the`target/debug` directory using the `dev` Cargo profile.                                                                                                       |
| 2.1 | `cargo +nightly build --release`                                                                                                                                                                    | Builds the project into the`target/release` directory using the `release` Cargo profile.                                                                                                 |
| 3   | `cargo +nightly run -- 2> stderr`                                                                                                                                                                   | Runs the debug executable, builds the project if necessary. stderr is redirected to the `stderr` file to catch panic messages, as per https://docs.rs/cursive/latest/cursive/#debugging. |
| 3.1 | `cargo +nightly run --release`                                                                                                                                                                      | Runs the release executable, builds the project if necessary.                                                                                                                            |
