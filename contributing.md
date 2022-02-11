# Contributor Guide

## Prepare the development environment

### Install the Rust toolchain

Install the [Rust](https://www.rust-lang.org/) toolchain
by installing [Rustup](https://www.rust-lang.org/tools/install).

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

| #   | Command                                                                             | Description                                                   |
|-----|-------------------------------------------------------------------------------------|---------------------------------------------------------------|
| 0   | `{ cargo clippy --all-targets --all-features ; cargo fmt --check ; } && cargo test` | Analyzes and reports errors, checks style, runs tests.        |
| 0.1 | `cargo fmt`                                                                         | Reformats the code using`rustfmt`.                            |
| 0.2 | `cargo clean`                                                                       | Deletes the`target` directory.                                |
| 1   | `cargo build`                                                                       | Builds the project into the`target` directory.                |
| 2   | `cargo run`                                                                         | Runs the project executable, builds the project if necessary. |
