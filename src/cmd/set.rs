use std::time::{Duration, Instant};

use bytes::Bytes;

use crate::{
    frame::Frame,
    parse::{Parse, ParseError},
};

#[derive(Debug)]
pub struct Set {
    key: String,
    value: Bytes,
    expire: Option<Duration>,
}

impl Set {
    pub fn new(key: impl ToString, value: Bytes, expire: Option<Duration>) -> Set {
        Set {
            key: key.to_string(),
            value,
            expire,
        }
    }

    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<Set> {
        let key = parse.next_string()?;
        let value = parse.next_bytes()?;
        let mut expire = None;

        match parse.next_string() {
            Ok(s) if s.to_uppercase() == "EX" => {
                let secs = parse.next_int()?;
                expire = Some(Duration::from_secs(secs));
            }
            Ok(s) if s.to_uppercase() == "PX" => {
                let ms = parse.next_int()?;
                expire = Some(Duration::from_millis(ms));
            }
            Ok(_) => return Err("currently `SET` only supports the expiration option".into()),
            Err(ParseError::EndOfStream) => {}
            Err(err) => return Err(err.into()),
        }

        Ok(Set { key, value, expire })
    }

    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("set".as_bytes()));
        frame.push_bulk(Bytes::from(self.key.into_bytes()));
        frame.push_bulk(self.value);

        if let Some(ms) = self.expire {
            frame.push_bulk(Bytes::from("px".as_bytes()));
            frame.push_bulk(Bytes::from(ms.as_millis().to_string()));
        }

        frame
    }

    pub(crate) fn apply_to_db(self, db: &crate::db::Db) {
        let expire = self.expire.map(|d| Instant::now() + d);
        db.set(&self.key, self.value, expire);
    }

    pub(crate) fn apply(
        self,
        db: &crate::db::Db,
        dst: &mut crate::connection::Connection,
    ) -> impl std::future::Future<Output = crate::Result<()>> {
        async move {
            self.apply_to_db(db);
            let response = Frame::Simple("OK".to_string());
            dst.write_frame(&response).await?;
            Ok(())
        }
    }
}
