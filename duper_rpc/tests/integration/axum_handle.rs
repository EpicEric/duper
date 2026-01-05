use std::{
    sync::{Arc, atomic::AtomicUsize},
    time::Duration,
};

use axum::{
    Router,
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::post,
};
use axum_duper::Duper;
use duper::{DuperKey, DuperParser, DuperValue};
use http_body_util::BodyExt;
use reqwest::{Method, header::CONTENT_TYPE};
use tower::{Service, ServiceExt};

#[tokio::test]
async fn axum_handle() {
    #[derive(Clone)]
    struct AppState {
        count: Arc<AtomicUsize>,
    }

    let mut app = Router::new()
        .route(
            "/rpc",
            post(
                |State(state): State<AppState>, Duper(request): Duper<duper_rpc::Request>| async move {
                    let Ok(response) = duper_rpc::Server::new()
                        .method(
                            "increment",
                            async |duper_rpc::State(state): duper_rpc::State<AppState>| {
                                Ok(state
                                    .count
                                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst))
                            },
                        )
                        .with_state(state)
                        .handle(request)
                        .await;
                    match response {
                        Some(response) => Duper(response).into_response(),
                        None => StatusCode::NO_CONTENT.into_response(),
                    }
                },
            ),
        )
        .with_state(AppState {
            count: Arc::default(),
        });

    let service = ServiceExt::<Request<Body>>::ready(&mut app).await.unwrap();

    let response_1 = service
        .call(
            Request::builder()
                .method(Method::POST)
                .uri("/rpc")
                .header(CONTENT_TYPE, "application/duper")
                .body(
                    r#"
                        RpcRequest({
                            duper_rpc: "0.1",
                            method: "increment",
                            id: 0,
                        })
                    "#
                    .to_string(),
                )
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response_1.status(), StatusCode::OK);
    let body = response_1.into_body().collect().await.unwrap().to_bytes();
    let value = DuperParser::parse_duper_trunk(str::from_utf8(body.as_ref()).unwrap()).unwrap();
    let DuperValue::Object { inner, .. } = value else {
        panic!("Invalid response {:?}", value);
    };
    assert_eq!(
        inner.get(&DuperKey::from("id")),
        Some(&DuperValue::Integer {
            identifier: None,
            inner: 0
        })
    );
    assert_eq!(
        inner.get(&DuperKey::from("result")),
        Some(&DuperValue::Integer {
            identifier: None,
            inner: 0
        })
    );

    let response_2 = service
        .call(
            Request::builder()
                .method(Method::POST)
                .uri("/rpc")
                .header(CONTENT_TYPE, "application/duper")
                .body(
                    r#"
                        RpcRequest({
                            duper_rpc: "0.1",
                            method: "increment",
                            // no id
                        })
                    "#
                    .to_string(),
                )
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response_2.status(), StatusCode::NO_CONTENT);
    tokio::time::sleep(Duration::from_millis(100)).await;

    let response_3 = service
        .call(
            Request::builder()
                .method(Method::POST)
                .uri("/rpc")
                .header(CONTENT_TYPE, "application/duper")
                .body(
                    r#"
                        RpcRequest({
                            duper_rpc: "0.1",
                            method: "increment",
                            id: SomeIdentifier(2),
                        })
                    "#
                    .to_string(),
                )
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response_3.status(), StatusCode::OK);
    let body = response_3.into_body().collect().await.unwrap().to_bytes();
    let value = DuperParser::parse_duper_trunk(str::from_utf8(body.as_ref()).unwrap()).unwrap();
    let DuperValue::Object { inner, .. } = value else {
        panic!("Invalid response {:?}", value);
    };
    assert_eq!(
        inner.get(&DuperKey::from("id")),
        Some(&DuperValue::Integer {
            identifier: None,
            inner: 2
        })
    );
    assert_eq!(
        inner.get(&DuperKey::from("result")),
        Some(&DuperValue::Integer {
            identifier: None, // Identifier is lost due to serde
            inner: 2
        })
    );
}
