use bytes::Bytes;

use crate::frame::Frame;

#[derive(Debug)]
pub struct Ping {
    msg: Option<Bytes>,
}

impl Ping {
    pub fn new(msg: Option<Bytes>) -> Self {
        Ping { msg }
    }

    pub(crate) fn parse_frames(parse: &mut crate::parse::Parse) -> crate::Result<Ping> {
        use crate::parse::ParseError::EndOfStream;

        let msg = match parse.next_string() {
            Ok(s) => Some(Bytes::from(s)),
            Err(EndOfStream) => None,
            Err(err) => return Err(err.into()),
        };
        Ok(Ping { msg })
    }

    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("ping"));
        if let Some(msg) = self.msg {
            frame.push_bulk(msg);
        }
        frame
    }

    pub(crate) fn apply(
        self,
        dst: &mut crate::connection::Connection,
    ) -> impl std::future::Future<Output = crate::Result<()>> {
        async move {
            let response = match self.msg {
                Some(msg) => Frame::Bulk(msg),
                None => Frame::Simple("PONG".to_string()),
            };
            dst.write_frame(&response).await?;
            Ok(())
        }
    }
}
