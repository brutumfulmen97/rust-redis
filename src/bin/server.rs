use tokio::net::TcpListener;

// type SharedDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;
// fn new_shared_db(num_shards: usize) -> SharedDb {
//     let mut db = Vec::with_capacity(num_shards);
//     for _ in 0..num_shards {
//         db.push(Mutex::new(HashMap::new()));
//     }
//     Arc::new(db)
// }

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    rust_redis::server::run(listener).await;
}
