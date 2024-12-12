pub mod chat;
pub mod client;
pub mod messages;
pub mod reader;
pub mod server;
pub mod writer;

use std::io;
use std::io::Read;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChatReaderError {
    #[error("io failed")]
    IO(#[from] io::Error),
    #[error("deserialization failed")]
    Serialization(#[from] serde_json::Error),
}

pub type ChatResult<T> = Result<T, ChatReaderError>;
