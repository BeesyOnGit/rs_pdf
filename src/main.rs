mod utils;

use axum::{Router, routing::post};
use tokio;
use utils::handler::handle_conversion;

#[tokio::main()]
async fn main() {
    let app = Router::new().route("/convert", post(handle_conversion));
    // .with_state(app_state);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3005").await.unwrap();

    // tracing::info!("Server running on {}", listener.local_addr().unwrap());
    println!("server listenig on port 3005");
    axum::serve(listener, app).await.unwrap();
}
