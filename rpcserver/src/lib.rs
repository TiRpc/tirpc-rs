use bincode;
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    sync::Mutex,
};

// TODO: remove all `unwrap`

pub const PACKETDELIMITER: u8 = '\n' as u8;
pub const FUNCTIONDELIMITER: u8 = '\\' as u8;

lazy_static! {
    static ref FUNCTIONS: Mutex<HashMap<String, Box<fn(Vec<u8>) -> Vec<u8>>>> =
        Mutex::new(HashMap::new());
}

pub fn register(fname: String, f: fn(Vec<u8>) -> Vec<u8>) {
    let mut guard = FUNCTIONS.lock().unwrap();
    guard.insert(fname, Box::new(f));
}

pub fn run(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        let mut ste = stream.unwrap();
        let mut req = vec![];
        let mut br = BufReader::new(&mut ste);
        br.read_until(PACKETDELIMITER, &mut req).unwrap();

        let resp = handle_call(req);
        ste.write(&resp).unwrap();
        ste.write(&[PACKETDELIMITER]).unwrap();
    }
}

fn handle_call(req: Vec<u8>) -> Vec<u8> {
    let mut idx = 0;
    while idx < req.len() {
        if req[idx] == FUNCTIONDELIMITER {
            break;
        }
        idx += 1;
    }
    let fname: String = bincode::deserialize(&req[..idx]).unwrap();
    let req = req[(idx + 1)..].to_vec();
    let guard = FUNCTIONS.lock().unwrap();
    let f = guard.get(&fname).unwrap();
    return f(req);
}
