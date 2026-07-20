use std::io::Cursor;

use bytes::Buf;
use tokio::{
    fs::{self, File},
    io::{AsyncWriteExt, BufWriter},
};

use crate::{cmd::Command, db::Db, frame::Frame};

#[derive(Debug)]
pub(crate) struct Wal {
    file: BufWriter<File>,
}

impl Wal {
    pub(crate) async fn open(path: &str) -> crate::Result<Wal> {
        if let Some(parent) = std::path::Path::new(path).parent() {
            fs::create_dir_all(parent).await?;
        }

        let file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .await?;

        Ok(Wal {
            file: BufWriter::new(file),
        })
    }

    pub(crate) async fn append(&mut self, data: &[u8]) -> crate::Result<()> {
        self.file.write_all(data).await?;
        self.file.flush().await?;
        Ok(())
    }

    pub(crate) async fn replay(db: &Db, path: &str) -> crate::Result<()> {
        let data = match fs::read(path).await {
            Ok(data) => data,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(e.into()),
        };

        let mut cursor = Cursor::new(&data[..]);
        while cursor.position() < data.len() as u64 {
            let mut buf = Cursor::new(&data[cursor.position() as usize..]);
            match Frame::check(&mut buf) {
                Ok(_) => {
                    let len = buf.position() as usize;
                    buf.set_position(0);
                    let frame = match Frame::parse(&mut buf) {
                        Ok(frame) => frame,
                        Err(_) => break,
                    };
                    if let Ok(cmd) = Command::from_frame(frame) {
                        let _ = cmd.apply_to_db(db);
                    }
                    cursor.advance(len);
                }
                Err(_) => break,
            }
        }

        Ok(())
    }
}
