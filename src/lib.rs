pub mod client;
pub mod cmd;
pub mod connection;
pub mod frame;
pub mod parse;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
