use crate::{frame::Frame, parse::Parse};

#[derive(Debug)]
pub struct Exists {
    keys: Vec<String>,
}

impl Exists {
    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<Exists> {
        use crate::parse::ParseError::EndOfStream;
        let mut keys = Vec::new();
        loop {
            match parse.next_string() {
                Ok(key) => keys.push(key),
                Err(EndOfStream) => break,
                Err(err) => return Err(err.into()),
            }
        }

        Ok(Exists { keys })
    }

    pub(crate) fn apply(
        self,
        db: &crate::db::Db,
        dst: &mut crate::connection::Connection,
    ) -> impl std::future::Future<Output = crate::Result<()>> {
        async move {
            let count = self.keys.iter().filter(|k| db.exists(k)).count() as u64;
            dst.write_frame(&Frame::Integer(count)).await?;
            Ok(())
        }
    }
}
