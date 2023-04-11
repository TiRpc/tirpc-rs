use bincode;

use def::{self, TiRpcError};
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

pub struct RpcClient {
    addr: String,
}

impl RpcClient {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn sendrpc(&self, req: Vec<u8>) -> Result<Vec<u8>, def::TiRpcError> {
        let mut conn = TcpStream::connect(&self.addr)?;
        conn.write(&req)?;
        conn.write(&[def::PACKETDELIMITER])?;
        conn.flush()?;
        let mut resp = vec![];
        let mut br = BufReader::new(&mut conn);
        br.read_until(def::PACKETDELIMITER, &mut resp)?;
        return Ok(resp);
    }
}

#[macro_export]
macro_rules! callrpc {
    ($param:expr) => {
        bincode::serialize(&$param).unwrap()
    };
    (fname $f:expr, client $cli: expr, $($params:expr),+) => {{
        let mut data = callrpc!($($params),+);
        let mut fb = bincode::serialize(&$f).unwrap();
        fb.push(def::FUNCTIONDELIMITER);
        fb.append(&mut data);
        $cli.sendrpc(fb).unwrap()
    }};
    ($param:expr, $($params:expr),+) =>{{
        let mut data = callrpc!($param);
        let mut rest = callrpc!($($params),+);
        data.append(&mut rest);
        data
    }}
}

pub fn deserialize<'a, T: serde::de::Deserialize<'a>>(data: &'a Vec<u8>) -> Result<T, TiRpcError> {
    Ok(bincode::deserialize::<T>(data)?)
}
