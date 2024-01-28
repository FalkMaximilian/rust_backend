use std::env;
use axum::{extract::{Path, Query}, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}, Router};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

use crate::model::ModelController;
pub use self::error::{Error, Result};

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    
    let port = env::var("PORT").expect("Missing port number");
    let port = port.parse::<u16>().expect("Invalid port given");

    let mc = ModelController::new().await?;

    let routes_api = web::routes_tickets::router(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service()).await.unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    
    println!();
    res
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
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