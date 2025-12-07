# Rust guide

Get started with Duper in Rust with one of the several crates available.

## Native type (de)serialization with `serde`

### Installation

```bash
cargo add serde_duper
```

### Quick example

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Product {
    name: String,
    price: (u32, u8),
    in_stock: bool,
    tags: Vec<String>,
}

fn main() -> serde_duper::Result<()> {
    let product = Product {
        name: "Wireless Headphones".to_string(),
        price: (129, 99),
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

### Basic serialization

```rust
use serde_duper::{to_string, to_string_pretty};

let data = vec!["hello", "world"];
let compact = to_string(&data)?;
let pretty = to_string_pretty(&data)?;
```

### Basic deserialization

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

### Working with identifiers

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

### Working with Temporal values

You can use `chrono`, or directly manipulate values with `TemporalString`:

```rust
use chrono::{DateTime, Utc};
use duper::DuperTemporal;
use serde::{Deserialize, Serialize};
use serde_duper::TemporalString;

#[derive(Serialize, Deserialize)]
struct DateValidator<'a> {
    #[serde(with = "serde_duper::types::chrono::DuperDateTime")]
    instant: DateTime<Utc>,
    matches: TemporalString<'a>,
}

let item = DateValidator {
    instant: "2023-10-05T14:30:00Z".parse().unwrap(),
    matches: TemporalString(DuperTemporal::try_plain_year_month_from(
        std::borrow::Cow::Borrowed("2023-10")
    ).unwrap()),
};

let output = serde_duper::to_string_pretty(&item)?;
// DateValidator({
//   instant: Instant('2023-10-05T14:30:00Z'),
//   matches: PlainYearMonth('2023-10'),
// })
```

### Working with bytes

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

---

See the [docs](https://docs.rs/serde_duper/latest/serde_duper/) for more information.

## HTTP requests/responses with `axum`

### Installation

```bash
cargo add axum_duper
```

### Quick example

```rust
use axum::{Router, routing::post};
use axum_duper::Duper;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
struct CreateUser {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct User {
    id: Uuid,
    email: String,
}

async fn create_user(Duper(payload): Duper<CreateUser>) -> Duper<User> {
    let user = new_user(payload).await.unwrap();
    Duper(user)
}

async fn new_user(payload: CreateUser) -> Result<User> {
    // ... add user to database ...
    Ok(user)
}

let app = Router::new().route("/user", post(create_user));
```

## Logging with `tracing`

### Installation

```bash
cargo add tracing_duper
```

### Quick example

```rust
use tracing::{debug, warn};
use tracing_duper::DuperLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tracing::instrument]
fn send_gifts(count: &mut usize) {
    if *count < 12 {
        warn!("too few gifts... try again later");
    } else {
        debug!(
            user_id = &b"santa"[..],
            "$duper.delivery_date" = "(PlainMonthDay('12-25'), \"Christmas\")",
            "sending {count} gifts"
        );
        std::thread::sleep(std::time::Duration::from_millis(100));
        *count = 0;
    }
}

fn main() {
    tracing_subscriber::registry()
        .with(DuperLayer::new().with_span_timings(true))
        .init();
    let mut gifts = 10;
    send_gifts(&mut gifts);
    gifts += 13;
    send_gifts(&mut gifts);
}
```
---

See the [docs](https://docs.rs/tracing_duper/latest/tracing_duper/) for more information.