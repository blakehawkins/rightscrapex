default:
  just --list

check: update cargo-check fmt clippy fmt test outdated

update:
  cargo update

cargo-check:
  cargo check

fmt:
  cargo fmt

clippy:
  cargo clippy -- --deny warnings

test:
  cargo test

outdated:
  cargo outdated -R
