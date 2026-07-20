use std::sync::Arc;

use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use crate::{cmd::Command, connection::Connection, db::Db, wal::Wal};

pub async fn run(listener: TcpListener) {
    let db = Arc::new(Db::new());
    let wal_path = "data/redis.wal";

    if let Err(e) = Wal::replay(&db, wal_path).await {
        eprintln!("WAL replay error: {e}");
    } else {
        println!("Replayed WAL");
    }

    let wal = Arc::new(Mutex::new(Wal::open(wal_path).await.unwrap()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        let wal = wal.clone();
        tokio::spawn(async move {
            if let Err(e) = process(socket, db, wal).await {
                eprintln!("connection error: {e}");
            }
        });
    }
}

async fn process(socket: TcpStream, db: Arc<Db>, wal: Arc<Mutex<Wal>>) -> crate::Result<()> {
    let mut connection = Connection::new(socket);

    while let Some((frame, raw)) = connection.read_frame_as_bytes().await? {
        let cmd = Command::from_frame(frame)?;

        if cmd.is_write() {
            let mut wal = wal.lock().await;
            wal.append(&raw).await?;
        }
        cmd.apply(&db, &mut connection).await?;
    }

    Ok(())
}
