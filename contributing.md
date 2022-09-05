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

## Build-related commands

This project uses [Cargo](https://doc.rust-lang.org/cargo/index.html) for build automation.

Run from the project root directory:

| #   | Command                                                                                                        | Description                                                                                                                    |
|-----|----------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------|
| 0   | `cargo clippy --all-targets --all-features && cargo fmt --check && cargo +nightly doc --no-deps -Zrustdoc-map` | Analyzes and reports errors, checks style, runs tests.                                                                         |
| 0.1 | `cargo fmt`                                                                                                    | Reformats the code using`rustfmt`.                                                                                             |
| 0.2 | `cargo clean`                                                                                                  | Deletes the`target` directory.                                                                                                 |
| 1   | `cargo +nightly doc --no-deps -Zrustdoc-map --open`                                                            | Generates documentation and opens it in a browser. See <https://doc.rust-lang.org/cargo/reference/unstable.html#rustdoc-map> . |
| 1.1 | `cargo clean --doc`                                                                                            | Deletes the `target/doc` directory.                                                                                            |
| 2   | `cargo build`                                                                                                  | Builds the project into the`target/debug` directory using the `dev` Cargo profile.                                             |
| 2.1 | `cargo build --release`                                                                                        | Builds the project into the`target/release` directory using the `release` Cargo profile.                                       |
| 3   | `cargo run`                                                                                                    | Runs the debug executable, builds the project if necessary.                                                                    |
| 3.1 | `cargo run --release`                                                                                          | Runs the release executable, builds the project if necessary.                                                                  |
