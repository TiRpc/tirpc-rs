use bincode;
use server;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

pub struct RPCClient{
    addr: String,
}

impl RPCClient{
    pub fn new(addr: String) -> Self{
        Self{ addr }
    }
    pub fn sendrpc(&self, req: Vec<u8>) -> Vec<u8> {
        let mut conn = TcpStream::connect(&self.addr).unwrap();
        conn.write(&req).unwrap();
        conn.write(&[server::PACKETDELIMITER]).unwrap();
        conn.flush().unwrap();
        let mut resp = vec![];
        let mut br = BufReader::new(&mut conn);
        br.read_until(server::PACKETDELIMITER, &mut resp).unwrap();
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
        fb.push(server::FUNCTIONDELIMITER);
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



// #[rpcfunc]
// fn foo(age: i32, name: String) -> String {
//     println!("I am {}, {} years old", name, age);
//     name
// }

// #[rpcfunc]
// fn bar(a: i32, b: i32) -> i32{
//     return a + b;
// }

// fn main() {
//     server::register("foo".to_string(), foo);
//     server::register("bar".into(), bar);
//     thread::spawn(move||{
//         server::callrpc();
//     });
//     thread::sleep(std::time::Duration::from_secs(1));
//     let resp = callrpc!(fname "bar".to_string(), 1, 8);
//     let ans: i32 = deserialize(&resp);
//     println!("{}", ans);
// }
