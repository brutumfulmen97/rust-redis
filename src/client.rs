use bytes::Bytes;
use tokio::net::TcpStream;

use crate::{
    Result,
    cmd::{Del, Get, Ping, Set},
    connection::Connection,
    frame::Frame,
};

#[derive(Debug)]
pub struct Client {
    connection: Connection,
}

impl Client {
    pub async fn connect(addr: &str) -> Result<Client> {
        let socket = TcpStream::connect(addr).await?;
        let connection = Connection::new(socket);
        Ok(Client { connection })
    }

    pub async fn get(&mut self, key: &str) -> Result<Option<Bytes>> {
        let frame = Get::new(key).into_frame();

        self.connection.write_frame(&frame).await?;

        match self.connection.read_frame().await? {
            Some(Frame::Simple(s)) => Ok(Some(s.into())),
            Some(Frame::Bulk(value)) => Ok(Some(value)),
            Some(Frame::Null) => Ok(None),
            Some(frame) => Err(frame.to_error()),
            None => Err("connection reset by server".into()),
        }
    }

    pub async fn set(&mut self, key: &str, value: Bytes) -> Result<()> {
        let frame = Set::new(key, value, None).into_frame();

        self.connection.write_frame(&frame).await?;

        match self.connection.read_frame().await? {
            Some(Frame::Simple(ref res)) if res == "OK" => Ok(()),
            Some(frame) => Err(frame.to_error()),
            None => Err("connection reset by server".into()),
        }
    }

    pub async fn ping(&mut self, msg: Option<Bytes>) -> Result<Bytes> {
        let frame = Ping::new(msg).into_frame();

        self.connection.write_frame(&frame).await?;

        match self.connection.read_frame().await? {
            Some(Frame::Simple(s)) => Ok(s.into()),
            Some(Frame::Bulk(msg)) => Ok(msg),
            Some(frame) => Err(frame.to_error()),
            None => Err("connection reset by server".into()),
        }
    }

    pub async fn del(&mut self, keys: Vec<String>) -> Result<u64> {
        let frame = Del::new(keys).into_frame();

        self.connection.write_frame(&frame).await?;

        match self.connection.read_frame().await? {
            Some(Frame::Integer(i)) => Ok(i),
            Some(frame) => Err(frame.to_error()),
            None => Err("connection reset by server".into()),
        }
    }
}
