use axum::{extract::Query, response::{Html, IntoResponse}, routing::get, Router};
use serde::Deserialize;

pub fn root() -> Router{
    Router::new()
        .route("/hello", get(handler_hello))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
	name: Option<String>,
}
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
	println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

	let name = params.name.as_deref().unwrap_or("World!");
	Html(format!("Hello <strong>{name}</strong>"))
}
