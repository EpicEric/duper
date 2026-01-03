<p align="center">
    <img src="https://duper.dev.br/logos/duper-400.png" alt="The Duper logo, with a confident spectacled mole wearing a flailing blue cape." /> <br>
</p>
<h1 align="center">Axum Duper</h1>

<p align="center">
    <a href="https://crates.io/crates/axum_duper"><img alt="crates.io version" src="https://img.shields.io/crates/v/axum_duper?style=flat&logo=rust&logoColor=white&label=axum_duper"></a>
    <a href="https://github.com/EpicEric/duper"><img alt="GitHub license" src="https://img.shields.io/github/license/EpicEric/duper"></a>
</p>

Duper extractor / response for `axum`.

[Check out the official website for Duper.](https://duper.dev.br)

## Installation

```bash
cargo add axum_duper
```

## Example

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
struct UserResponse {
    id: Uuid,
}

async fn create_user(Duper(payload): Duper<CreateUser>) -> impl IntoResponse {
    let id = Uuid::new_v4();
    add_user_to_db(payload).await;
    Duper(UserResponse { id })
}

async fn add_user_to_db(user: CreateUser) {
    // ...
}

let app = Router::new().route("/users", post(create_user));
```
