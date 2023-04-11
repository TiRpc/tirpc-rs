use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TiRpcError {
    #[error("tcp io error")]
    TcpStreamError(#[from] io::Error),

    #[error("lock/unlock error `{0}`")]
    MutexError(String),

    #[error("serialize/deserialize error")]
    CodecError(#[from] bincode::Error),

    #[error("method not found")]
    MethodNotFound,

    #[error("unknown error")]
    Unknown,
}

pub const PACKETDELIMITER: u8 = '\n' as u8;
pub const FUNCTIONDELIMITER: u8 = '\\' as u8;
