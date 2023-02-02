mod routes;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use sea_orm::Database;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_uri: &str = dotenv!("DATABASE_URL");
    let database = Database::connect(database_uri).await;

    let app = routes::create_router();
    let addr: SocketAddr = "[::]:8080".parse().unwrap();

    println!("listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
