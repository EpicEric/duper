<p align="center">
    <img src="https://duper.dev.br/logos/duper-400.png" alt="The Duper logo, with a confident spectacled mole wearing a flailing blue cape." /> <br>
</p>
<h1 align="center">duper_rpc</h1>

<p align="center">
    <a href="https://crates.io/crates/duper_rpc"><img alt="Crates.io version" src="https://img.shields.io/crates/v/duper_rpc?style=flat&logo=rust&logoColor=white&label=duper_rpc"></a>
    <a href="https://github.com/EpicEric/duper"><img alt="GitHub license" src="https://img.shields.io/github/license/EpicEric/duper"></a>
</p>

An RPC implementation for Duper.

[Check out the official website for Duper.](https://duper.dev.br)

## Example with Axum

```bash
cargo add axum_duper duper_rpc
```

```rust
use axum::{extract::State, Router, routing::post};
use axum_duper::Duper;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Params {
    text: String,
}

#[derive(Clone)]
struct AppState(u64);

async fn handle_only_state(duper_rpc::State(state): duper_rpc::State<AppState>) -> duper_rpc::Result<u64> {
    Ok(state.0)
}

async fn handle_params(params: Params, flag: bool) -> duper_rpc::Result<String> {
    Ok(if flag { params.text } else { "flag is false".into() })
}

async fn rpc_handler(State(state): State<AppState>, request: Duper(duper_rpc::Request)) -> impl IntoResponse {
    let Ok(response) = duper_rpc::server()
        .method("foo", handle_only_state)
        .method("bar", handle_params)
        .method("healthy", async || Ok(true))
        .with_state(state)
        .handle(request)
        .await;
    response.map(Duper)
}

let app = Router::new().route("/rpc", post(rpc_handler)).with_state(AppState(42));
```
