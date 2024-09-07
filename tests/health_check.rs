//! tests/health_check.rs

use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let ip = "127.0.0.1";
    let listener = TcpListener::bind(format!("{ip}:0")).expect("Unable to bind to random port");
    let socket_addr = listener
        .local_addr()
        .expect("Unable to extract socket_addr");
    let port = socket_addr.port();

    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://{ip}:{port}")
}
