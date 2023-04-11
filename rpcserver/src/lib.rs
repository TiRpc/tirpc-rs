pub mod tuplecaller;

use def::{self, TiRpcError};
use bincode;
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    sync::Mutex,
};
pub use tuplecaller::TupleCaller;

// TODO: remove all `unwrap`


lazy_static! {
    static ref FUNCTIONS: Mutex<HashMap<String, Box<fn(Vec<u8>) -> Result<Vec<u8>, TiRpcError>>>> =
        Mutex::new(HashMap::new());
}

pub fn register(fname: String, f: fn(Vec<u8>) -> Result<Vec<u8>, TiRpcError>) ->Result<(), TiRpcError> {
    let mut guard = FUNCTIONS.lock()
                        .map_err(|e|TiRpcError::MutexError(e.to_string()))?;
    guard.insert(fname, Box::new(f));
    Ok(())
}

pub fn run(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        let mut ste = stream.unwrap();
        let mut req = vec![];
        let mut br = BufReader::new(&mut ste);
        br.read_until(def::PACKETDELIMITER, &mut req).unwrap();

        let resp = handle_call(req).unwrap();
        ste.write(&resp).unwrap();
        ste.write(&[def::PACKETDELIMITER]).unwrap();
    }
}

fn handle_call(req: Vec<u8>) -> Result<Vec<u8>, TiRpcError> {
    let mut idx = 0;
    while idx < req.len() {
        if req[idx] == def::FUNCTIONDELIMITER {
            break;
        }
        idx += 1;
    }
    let fname: String = bincode::deserialize(&req[..idx])?;
    let req = req[(idx + 1)..].to_vec();
    let guard = FUNCTIONS.lock().map_err(|e|TiRpcError::MutexError(e.to_string()))?;
    let f = guard.get(&fname).ok_or(TiRpcError::MethodNotFound)?;
    return f(req);
}
