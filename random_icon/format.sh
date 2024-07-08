#!/bin/bash
set -eux

cargo fmt
cargo clippy
