#!/bin/sh

set -ev

cargo test --all --verbose

# Skip formatting generate_reflection because of
# https://github.com/rust-lang/rustfmt/issues/3688
cargo fmt --package rbx_binary --package rbx_dom_weak --package rbx_reflection --package rbx_xml -- --check

cargo clippy
