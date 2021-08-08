#!/usr/bin/env bash

echo "'rustup' info"
rustup --version
echo ""
echo "Active and installed toolchains or profiles"
rustup show
echo ""
echo "Installing components"
echo "Installing 'rustfmt'"
rustup component add rustfmt
echo "Installing 'clippy'"
rustup component add clippy
echo "Finished installing components"
