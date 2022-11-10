#!/usr/bin/env bash

set -eu

cargo +nightly contract build --manifest-path ticket/Cargo.toml
cargo +nightly contract build