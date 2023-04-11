pub mod tuplecaller;

use bincode;
use def::{self, TiRpcError};
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    sync::Mutex,
};
pub use tuplecaller::TupleCaller;

lazy_static! {
    static ref FUNCTIONS: Mutex<HashMap<String, Box<fn(Vec<u8>) -> Result<Vec<u8>, TiRpcError>>>> =
        Mutex::new(HashMap::new());
}

pub fn register(
    fname: String,
    f: fn(Vec<u8>) -> Result<Vec<u8>, TiRpcError>,
) -> Result<(), TiRpcError> {
    let mut guard = FUNCTIONS
        .lock()
        .map_err(|e| TiRpcError::MutexError(e.to_string()))?;
    guard.insert(fname, Box::new(f));
    Ok(())
}

pub fn run<F: Fn(TiRpcError)>(addr: &str, callback: F) -> Result<(), TiRpcError> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        let rste = stream.map_err(|e| callback(TiRpcError::TcpStreamError(e)));
        if !rste.is_ok() {
            continue;
        }
        let mut ste = rste.unwrap();
        let mut req = vec![];
        let mut br = BufReader::new(&mut ste);
        if let Err(e) = br.read_until(def::PACKETDELIMITER, &mut req) {
            callback(TiRpcError::TcpStreamError(e));
            continue;
        }

        match handle_call(req) {
            Ok(resp) => {
                if let Err(e) = ste.write(&resp) {
                    callback(TiRpcError::TcpStreamError(e));
                    continue;
                };
                if let Err(e) = ste.write(&[def::PACKETDELIMITER]) {
                    callback(TiRpcError::TcpStreamError(e));
                    continue;
                };
            }
            Err(e) => callback(e),
        };
    }
    Ok(())
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
    let guard = FUNCTIONS
        .lock()
        .map_err(|e| TiRpcError::MutexError(e.to_string()))?;
    let f = guard.get(&fname).ok_or(TiRpcError::MethodNotFound)?;
    return f(req);
}
