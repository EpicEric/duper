# Rust guide

Get started with Duper in Rust with the [`serde_duper`](https://crates.io/crates/serde_duper) crate.

## Installation

```bash
cargo add serde_duper
```

## Quick example

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Product {
    name: String,
    price: f64,
    in_stock: bool,
    tags: Vec<String>,
}

fn main() -> serde_duper::Result<()> {
    let product = Product {
        name: "Wireless Headphones".to_string(),
        price: 129.99,
        in_stock: true,
        tags: vec!["electronics".into(), "audio".into()],
    };

    // Convert to Duper format
    let duper_string = serde_duper::to_string_pretty(&product)?;
    println!("{}", duper_string);

    // Convert back from Duper
    let deserialized: Product = serde_duper::from_string(&duper_string)?;
    Ok(())
}
```

## Basic serialization

```rust
use serde_duper::{to_string, to_string_pretty};

let data = vec!["hello", "world"];
let compact = to_string(&data)?;
let pretty = to_string_pretty(&data)?;
```

## Basic deserialization

```rust
use serde_duper::from_string;

let duper_data = r#"
    {
        name: "John Doe",
        age: 30,
        active: true
    }
"#;

#[derive(Deserialize)]
struct User {
    name: String,
    age: u8,
    active: bool,
}

let user: User = from_string(duper_data)?;
```

## Using identifiers

`serde_duper` has special support for identifiers with Serde. Add or modify type hints to your serialized models:

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename = "Product")]  // Renames wrapper to "Product(...)"
struct Item {
    name: String,
    #[serde(with = "serde_duper::types::DuperUuid")]
    id: uuid::Uuid,  // Adds "Uuid(...)" wrapper
}

let item = Item {
    name: "Sample".to_string(),
    id: uuid::Uuid::new_v4(),
};

let output = serde_duper::to_string_pretty(&item)?;
// Product({
//   name: "Sample",
//   id: Uuid("a1b2c3d4-..."),
// })
```

## Working with bytes

You can make use of Duper's bytes support:

```rust
use serde_duper::bytes::{ByteBuf};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct FileData {
    metadata: String,
    #[serde(with = "serde_duper::bytes")]
    content: Vec<u8>,  // Better than Vec<u8> for serialization
}

// Or use the convenience types directly:
#[derive(Serialize, Deserialize)]
struct ImageData {
    format: String,
    data: ByteBuf,  // Equivalent to Vec<u8> but with better Duper support
}
```

Also, tuples are first-class values in `serde_duper`.
