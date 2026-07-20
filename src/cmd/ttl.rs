use std::time::Instant;

use crate::{connection::Connection, db::Db, frame::Frame, parse::Parse};

#[derive(Debug)]
pub struct Ttl {
    key: String,
}

impl Ttl {
    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<Ttl> {
        let key = parse.next_string()?;
        Ok(Ttl { key })
    }

    pub(crate) fn apply(
        self,
        db: &Db,
        dst: &mut Connection,
    ) -> impl std::future::Future<Output = crate::Result<()>> {
        async move {
            match db.ttl(&self.key, Instant::now()) {
                Some(remaining) => {
                    let secs = remaining.as_secs() as i64;
                    dst.write_frame(&Frame::Integer(secs as u64)).await?;
                }
                None => {
                    dst.write_frame(&Frame::Integer(0)).await?;
                }
            }
            Ok(())
        }
    }
}
