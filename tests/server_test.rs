use std::time::Duration;

use rust_redis::client::Client;
use rust_redis::server;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

async fn start_server() -> (u16, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let handle = tokio::spawn(async move {
        server::run(listener).await;
    });
    tokio::time::sleep(Duration::from_millis(300)).await;
    (port, handle)
}

#[tokio::test]
async fn set_and_get() {
    let (port, _server) = start_server().await;
    let addr = format!("127.0.0.1:{port}");
    let mut client = Client::connect(&addr).await.unwrap();
    client.set("foo", "bar".into()).await.unwrap();
    let value = client.get("foo").await.unwrap().unwrap();
    assert_eq!(value, "bar");
}

#[tokio::test]
async fn get_missing_key() {
    let (port, _server) = start_server().await;
    let addr = format!("127.0.0.1:{port}");

    let mut client = Client::connect(&addr).await.unwrap();
    let value = client.get("missing").await.unwrap();
    assert_eq!(value, None);
}

#[tokio::test]
async fn ping() {
    let (port, _server) = start_server().await;
    let addr = format!("127.0.0.1:{port}");

    let mut client = Client::connect(&addr).await.unwrap();
    let response = client.ping(None).await.unwrap();
    assert_eq!(response, "PONG");
}
