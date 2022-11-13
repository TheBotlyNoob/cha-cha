use hyper::Server;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    let port = args.next().and_then(|p| p.parse().ok()).unwrap_or(8080);

    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let svc = tower_http::services::ServeDir::new(args.next().unwrap_or_else(|| String::from(".")));

    let server = Server::bind(&addr).serve(tower::make::Shared::new(svc));

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
