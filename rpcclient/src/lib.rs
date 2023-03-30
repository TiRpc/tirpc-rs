use bincode;
use rpcserver;
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
    pub fn sendrpc(&self, req: Vec<u8>) -> Vec<u8> {
        let mut conn = TcpStream::connect(&self.addr).unwrap();
        conn.write(&req).unwrap();
        conn.write(&[rpcserver::PACKETDELIMITER]).unwrap();
        conn.flush().unwrap();
        let mut resp = vec![];
        let mut br = BufReader::new(&mut conn);
        br.read_until(rpcserver::PACKETDELIMITER, &mut resp)
            .unwrap();
        return resp;
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
        fb.push(rpcserver::FUNCTIONDELIMITER);
        fb.append(&mut data);
        $cli.sendrpc(fb)
    }};
    ($param:expr, $($params:expr),+) =>{{
        let mut data = callrpc!($param);
        let mut rest = callrpc!($($params),+);
        data.append(&mut rest);
        data
    }}
}

pub fn deserialize<'a, T: serde::de::Deserialize<'a>>(data: &'a Vec<u8>) -> T {
    bincode::deserialize::<T>(data).unwrap()
}
