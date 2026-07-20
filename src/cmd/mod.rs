mod get;
pub use get::Get;

mod set;
pub use set::Set;

use crate::{frame::Frame, parse::Parse};

#[derive(Debug)]
pub enum Command {
    Get(Get),
    Set(Set),
}

impl Command {
    pub fn from_frame(frame: Frame) -> crate::Result<Command> {
        let mut parse = Parse::new(frame)?;
        let command_name = parse.next_string()?.to_lowercase();
        let command = match &command_name[..] {
            "get" => Command::Get(Get::parse_frames(&mut parse)?),
            "set" => Command::Set(Set::parse_frames(&mut parse)?),
            _ => return Err(format!("unknown command: {command_name}").into()),
        };
        parse.finish()?;
        Ok(command)
    }
}
