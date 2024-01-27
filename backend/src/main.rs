use std::env;

use axum::{extract::{Path, Query}, response::{Html, IntoResponse}, routing::get, Router};
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    
    let port = env::var("PORT").expect("Missing port number");
    let port = port.parse::<u16>().expect("Invalid port given");

    let routes_hello = Router::new()
        .route("/", get(handler_root))
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2));

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_hello.into_make_service()).await.unwrap();
}

async fn handler_root() -> impl IntoResponse {
    println!("->> {:<12} - handler_root", "HANDLER");

    Html("HiHi Hallo :)")
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// E.g. '/hello?name=Jen'
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// E.g. '/hello2/Max'
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}