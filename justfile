default:
  just --list

test: test-rust test-python

test-rust:
  cargo test --all-features

[working-directory: 'duper-python']
test-python:
  uv run maturin develop
  uv run pytest -vv

clippy:
  cargo clippy --all-features --all-targets --fix --allow-dirty --allow-staged && cargo fmt --all
