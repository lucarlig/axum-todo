mod routes;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = routes::create_router();
    let addr: SocketAddr = "[::]:8080".parse().unwrap();

    println!("listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
