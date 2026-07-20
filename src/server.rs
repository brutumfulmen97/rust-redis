use std::sync::Arc;

use tokio::net::{TcpListener, TcpStream};

use crate::{cmd::Command, connection::Connection, db::Db};

pub async fn run(listener: TcpListener) {
    let db = Arc::new(Db::new());

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        tokio::spawn(async move {
            if let Err(e) = process(socket, db).await {
                eprintln!("connection error: {e}");
            }
        });
    }
}

async fn process(socket: TcpStream, db: Arc<Db>) -> crate::Result<()> {
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await? {
        let cmd = Command::from_frame(frame)?;
        cmd.apply(&db, &mut connection).await?;
    }

    Ok(())
}
