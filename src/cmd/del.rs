use bytes::Bytes;

use crate::{frame::Frame, parse::Parse};

#[derive(Debug)]
pub struct Del {
    keys: Vec<String>,
}

impl Del {
    pub fn new(keys: Vec<String>) -> Self {
        Self { keys }
    }

    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<Del> {
        use crate::parse::ParseError::EndOfStream;
        let mut keys = Vec::new();
        loop {
            match parse.next_string() {
                Ok(key) => keys.push(key),
                Err(EndOfStream) => break,
                Err(err) => return Err(err.into()),
            }
        }

        Ok(Del { keys })
    }

    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("del"));
        for key in self.keys {
            frame.push_bulk(Bytes::from(key));
        }
        frame
    }

    pub(crate) fn apply_to_db(self, db: &crate::db::Db) -> u64 {
        let count = self.keys.iter().filter(|k| db.del(k)).count() as u64;
        count
    }

    pub(crate) fn apply(
        self,
        db: &crate::db::Db,
        dst: &mut crate::connection::Connection,
    ) -> impl std::future::Future<Output = crate::Result<()>> {
        async move {
            let count = self.apply_to_db(db);
            dst.write_frame(&Frame::Integer(count)).await?;
            Ok(())
        }
    }
}
