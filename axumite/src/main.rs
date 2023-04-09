#![allow(unused)]

use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(|| async { "Hello" }));

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    print!("Listening on http://{}", address);
    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
