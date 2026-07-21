mod get;
pub use get::Get;
mod set;
pub use set::Set;
mod ping;
pub use ping::Ping;
mod del;
pub use del::Del;
mod exists;
pub use exists::Exists;
mod expire;
pub use expire::Expire;
mod ttl;
pub use ttl::Ttl;

use crate::{connection::Connection, db::Db, frame::Frame, parse::Parse};

#[derive(Debug)]
pub enum Command {
    Get(Get),
    Set(Set),
    Del(Del),
    Exists(Exists),
    Ping(Ping),
    Ttl(Ttl),
    Expire(Expire),
}

impl Command {
    pub fn from_frame(frame: Frame) -> crate::Result<Command> {
        let mut parse = Parse::new(frame)?;
        let command_name = parse.next_string()?.to_lowercase();
        let command = match &command_name[..] {
            "get" => Command::Get(Get::parse_frames(&mut parse)?),
            "set" => Command::Set(Set::parse_frames(&mut parse)?),
            "del" => Command::Del(Del::parse_frames(&mut parse)?),
            "ping" => Command::Ping(Ping::parse_frames(&mut parse)?),
            "exists" => Command::Exists(Exists::parse_frames(&mut parse)?),
            "ttl" => Command::Ttl(Ttl::parse_frames(&mut parse)?),
            "expire" => Command::Expire(Expire::parse_frames(&mut parse)?),
            _ => return Err(format!("unknown command: {command_name}").into()),
        };
        parse.finish()?;
        Ok(command)
    }

    pub(crate) async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        match self {
            Command::Del(cmd) => cmd.apply(db, dst).await,
            Command::Exists(cmd) => cmd.apply(db, dst).await,
            Command::Get(cmd) => cmd.apply(db, dst).await,
            Command::Ping(cmd) => cmd.apply(dst).await,
            Command::Set(cmd) => cmd.apply(db, dst).await,
            Command::Ttl(cmd) => cmd.apply(db, dst).await,
            Command::Expire(cmd) => cmd.apply(db, dst).await,
        }
    }

    pub(crate) fn apply_to_db(self, db: &Db) -> crate::Result<()> {
        match self {
            Command::Del(cmd) => {
                cmd.apply_to_db(db);
            }
            Command::Expire(cmd) => {
                cmd.apply_to_db(db);
            }
            Command::Set(cmd) => {
                cmd.apply_to_db(db);
            }
            _ => {}
        }
        Ok(())
    }

    pub(crate) fn is_write(&self) -> bool {
        match self {
            Command::Set(_) | Command::Del(_) | Command::Expire(_) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::Frame;
    use bytes::Bytes;

    #[test]
    fn parse_get_command() {
        let frame = Frame::Array(vec![Frame::Bulk("GET".into()), Frame::Bulk("foo".into())]);
        let cmd = Command::from_frame(frame).unwrap();
        match cmd {
            Command::Get(get) => assert_eq!(get.key(), "foo"),
            _ => panic!("expected Get command"),
        }
    }

    #[test]
    fn parse_set_command() {
        let frame = Frame::Array(vec![
            Frame::Bulk("SET".into()),
            Frame::Bulk("key".into()),
            Frame::Bulk("value".into()),
        ]);
        let cmd = Command::from_frame(frame).unwrap();
        match cmd {
            Command::Set(set) => {
                assert_eq!(set.key(), "key");
                assert_eq!(set.value(), &Bytes::from("value"));
            }
            _ => panic!("expected Set command"),
        }
    }

    #[test]
    fn parse_unknown_command() {
        let frame = Frame::Array(vec![Frame::Bulk("not_a_command".into())]);
        assert!(Command::from_frame(frame).is_err());
    }
}
