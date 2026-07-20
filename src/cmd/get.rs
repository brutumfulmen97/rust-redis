use bytes::Bytes;

use crate::{frame::Frame, parse::Parse};

#[derive(Debug)]
pub struct Get {
    key: String,
}

impl Get {
    pub fn new(key: impl ToString) -> Get {
        Get {
            key: key.to_string(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<Get> {
        let key = parse.next_string()?;

        Ok(Get { key })
    }

    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("get".as_bytes()));
        frame.push_bulk(Bytes::from(self.key.into_bytes()));
        frame
    }

    pub(crate) fn apply(
        self,
        db: &crate::db::Db,
        dst: &mut crate::connection::Connection,
    ) -> impl std::future::Future<Output = crate::Result<()>> {
        async move {
            let response = if let Some(value) = db.get(&self.key()) {
                Frame::Bulk(value)
            } else {
                Frame::Null
            };

            dst.write_frame(&response).await?;
            Ok(())
        }
    }
}
