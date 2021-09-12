# Contributor Guide

## Prepare the development environment

### Install the Rust toolchain

Install the [Rust](https://www.rust-lang.org/) toolchain
by installing [`rustup`](https://www.rust-lang.org/tools/install).

### Setup the Rust toolchain

If you are using a shell capable of running
[Bash](https://www.gnu.org/software/bash/) scripts,
run the following from the project root directory:
```shell
$ ./setup.sh
```
Otherwise follow these manual steps:

1. Install [`rustfmt`](https://github.com/rust-lang/rustfmt).
2. Install [`clippy`](https://github.com/rust-lang/rust-clippy).

## Build-related commands

This project uses [Cargo](https://doc.rust-lang.org/cargo/index.html) for build automation.

Run from the project root directory:

&#x23; | Command | Description
--- | --- | ---
0 | `cargo check ; rustfmt --check src/main.rs ; cargo clippy -- -D clippy::cargo` | Analyze and report errors, check style.
0.1 | `cargo fmt` | Reformat the code using `rustfmt`.
0.2 | `cargo clean` | Delete the `target` directory.
1 | `cargo build` | Build the project into the `target` directory.
2 | `cargo run` | Run the project executable, build the project if necessary.
