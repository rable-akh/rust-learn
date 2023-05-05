
use std::net::SocketAddr;

use axum::extract::{Query, Path};
use axum::{Router, middleware};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};

use serde::Deserialize;
use tower_cookies::{CookieManagerLayer};
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};

mod error;
mod web;
mod model;

#[tokio::main]
async fn main() {
    let routes_hello: Router = Router::new()
                    .merge(hello_route())
                    .merge(web::routes_login::routes())
                    .layer(middleware::map_response(main_response_mapper))
                    .layer(CookieManagerLayer::new())
                    .fallback_service(route_static());

    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!(">> Listing {addr}");

    axum::Server::bind(&addr)
		.serve(routes_hello.into_make_service())
		.await
		.unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!(" >> Middware log response");

    res
}

fn route_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn hello_route() -> Router {
    Router::new()
    .route("/hello/:name", get(handaller_hello2))
    .route( "/hello", get(handler_hello) )
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handaller_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!(">> {:<12} - Handaller hello ", "Handaler");

    // let namer = params.name.as_deref().unwrap_or("Blar");

    return Html(format!("Hello <b>Rust developer Params: {name}</b>"));
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!(">> {:<12} - Handaller hello ", "Handaler");

    let name = params.name.as_deref().unwrap_or("Blar");

    return Html(format!("Hello <b>Rust developer Params: {name}</b>"));
}