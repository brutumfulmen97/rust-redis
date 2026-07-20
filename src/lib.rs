pub mod client;
pub(crate) mod cmd;
pub mod connection;
pub(crate) mod db;
pub mod frame;
pub mod parse;
pub mod server;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
