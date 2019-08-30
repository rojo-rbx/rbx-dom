#!/bin/sh

set -ev

cargo test --all --locked --verbose

# Skip formatting generate_reflection because of
# https://github.com/rust-lang/rustfmt/issues/3688
cargo fmt --package rbx_binary rbx_dom_weak rbx_reflection rbx_xml -- --check

cargo clippy