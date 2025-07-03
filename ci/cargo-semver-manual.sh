#!/bin/sh

# Work around https://github.com/obi1kenobi/cargo-semver-checks/issues/1068 to
# run semver checks on non-host targets.

set -eux

target="$1"
rustflags="--cap-lints=allow ${RUSTFLAGS:-}"
rustdocflags="-Z unstable-options --document-private-items \
    --document-hidden-items --output-format=json ${RUSTDOCFLAGS:-}"

RUSTC_BOOTSTRAP=1 RUSTFLAGS="$rustflags" RUSTDOCFLAGS="$rustdocflags" \
    cargo doc -p libc --target "$target"
