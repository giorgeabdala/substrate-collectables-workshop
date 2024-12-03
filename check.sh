#!/bin/bash

cargo +nightly fmt
cargo +nightly clippy
cargo test