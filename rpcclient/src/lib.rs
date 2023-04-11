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
        bincode::serialize(&$param).map_err(|e|TiRpcError::CodecError(e))
    };
    (fname $f:expr, client $cli: expr, $($params:expr),+) => {{
        let rfb = bincode::serialize(&$f);
        if let Err(e) = rfb{
            Err(TiRpcError::CodecError(e))
        }else{
            let mut fb = rfb.unwrap();
            let rdata: Result<Vec<u8>, TiRpcError> = callrpc!($($params),+);
            if let Err(e) = rdata{
                Err(e)
            }else{
                let mut data = rdata.unwrap();
                fb.push(def::FUNCTIONDELIMITER);
                fb.append(&mut data);
                $cli.sendrpc(fb)
            }
        }
    }};
    ($param:expr, $($params:expr),+) =>{{
        let rdata: Result<Vec<u8>, TiRpcError> = callrpc!($param);
        if let Err(e) = rdata{
            Err(e)
        }else{
            let mut data = rdata.unwrap();
            let rrest = callrpc!($($params),+);
            if let Err(e) = rrest{
                Err(e)
            }else{
                let mut rest = rrest.unwrap();
                data.append(&mut rest);
                Ok(data)
            }
        }
    }}
}

pub fn deserialize<'a, T: serde::de::Deserialize<'a>>(data: &'a Vec<u8>) -> Result<T, TiRpcError> {
    Ok(bincode::deserialize::<T>(data)?)
}
