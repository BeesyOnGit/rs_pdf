mod utils;

use axum::{Router, routing::post};
use utils::handler::handle_conversion;

#[tokio::main]
async fn main() {
    // Create router with conversion endpoint
    let app = Router::new()
        .route("/convert", post(handle_conversion));

    // Bind server to address
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3005")
        .await
        .expect("Failed to bind to port 3005");

    println!("Server listening on http://0.0.0.0:3005");
    println!("POST /convert - Convert HTML to PDF");

    // Start server
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
