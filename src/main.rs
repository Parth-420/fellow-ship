mod handlers;
mod models;
mod routes;


use std::net::SocketAddr;
use tokio::net::TcpListener;
use axum::serve;

#[tokio::main]
async fn main() {
    let app = routes::create_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
