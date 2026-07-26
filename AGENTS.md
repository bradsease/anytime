# Repository Guide

## Overview

`anytime` is a Rust library for astronomical time scales and conversions with nanosecond resolution. Core crate code is in `src/`, and bundled reference data is in `data/`.

## Development

- Use Cargo and stable Rust for code execution and testing; maintain compatibility with the minimum supported Rust version, 1.70.
- Run tests with `cargo test --workspace --all-features --all-targets`.
- Check formatting with `cargo fmt --all -- --check`.
- Run lint checks with `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Build documentation with `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps`.
- Verify the minimum supported Rust version with `cargo test --locked --workspace --all-features --all-targets` using Rust 1.70.
- Use rustfmt for all Rust formatting; do not introduce unrelated formatting changes.
- Update the relevant documentation and doctests when adding features or changing public interfaces; do not make unrelated documentation changes.
- Keep the README examples synchronized with the corresponding examples in `src/lib.rs`.
- Keep astronomical reference data and validation outputs consistent when changing time-scale or Earth-orientation calculations.

## Pull Requests

- Verify the final PR title and description against `git diff` and the complete commit range before submitting or updating a PR.
- Use real Markdown line breaks and describe every included change and verification result accurately.
- Request review from Brad Sease (`bradsease`) when creating a PR.
