# duperfmt

`duperfmt` is a Duper formatter based on tree-sitter and Topiary. It powers `duper_lsp`'s own formatting engine.

## Installation

`duperfmt` is currently only available as a crates.io binary. To compile from source:

```bash
cargo install --locked duperfmt
```

## Basic usage

```bash
duperfmt -f input.duper -o output.duper
```

Run `duperfmt --help` for more details.
