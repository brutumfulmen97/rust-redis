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
}
