use std::time::{Duration, Instant};

use crate::{connection::Connection, db::Db, frame::Frame, parse::Parse};

#[derive(Debug)]
pub struct Expire {
    key: String,
    seconds: u64,
}

impl Expire {
    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<Expire> {
        let key = parse.next_string()?;
        let seconds = parse.next_int()?;
        Ok(Expire { key, seconds })
    }

    pub(crate) fn apply(
        self,
        db: &Db,
        dst: &mut Connection,
    ) -> impl std::future::Future<Output = crate::Result<()>> {
        async move {
            let expires_at = Instant::now() + Duration::from_secs(self.seconds);
            let ok = db.expire(&self.key, expires_at);
            dst.write_frame(&Frame::Integer(if ok { 1 } else { 0 }))
                .await?;
            Ok(())
        }
    }
}
