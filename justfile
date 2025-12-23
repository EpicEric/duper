default:
    just --list

test: test-rust test-python test-wasm

test-rust:
    cargo nextest run --all-features

[working-directory('duper-python')]
test-python:
    uv run maturin develop
    uv run pytest -v

[working-directory('duper-js-wasm')]
test-wasm:
    npm install
    npm run build
    npm run test

lint: lint-rust lint-python

alias clippy := lint-rust

lint-rust:
    cargo clippy --all-features --all-targets --fix --allow-dirty --allow-staged && cargo fmt --all

[working-directory('duper-python')]
lint-python:
    uv run ruff format
