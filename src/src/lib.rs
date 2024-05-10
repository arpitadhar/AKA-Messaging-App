use router::create_router;
use tracing::info;
pub use self::error::{Error, Result};
use tracing_subscriber::FmtSubscriber;

mod router;
mod routes;
mod database;
mod error;

pub async fn run() {
    tracing::subscriber::set_global_default(FmtSubscriber::default());
    let app = create_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, app).await.unwrap();
    info!("Starting Server!");
}