#!/usr/bin/env bash

printf "Rustup info:\n"
rustup --version
printf "\nActive and installed toolchains or profiles:\n"
rustup show
printf "Installing components."
printf "\n\nInstalling Rustfmt."
rustup component add rustfmt
printf "\nInstalling Clippy."
rustup component add clippy
printf "\nInstalling Cargo plugins."
printf "\n\nInstalling cargo-deny."
cargo install cargo-deny --locked
printf "\nInstalling cargo-udeps."
cargo install cargo-udeps --locked
printf "\nIf an installation of the pugins cargo-deny, cargo-udeps fails,\
\nyou may need to install pkg-config, libssl-dev. For example:\
\n$ sudo apt-get install pkg-config && sudo apt-get install libssl-dev\n"
