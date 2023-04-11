use bincode;
use def::TiRpcError;
use macros::rpcfunc;
use rpcclient::{self, callrpc};
use rpcserver::TupleCaller;
use serde::{Deserialize, Serialize};
use std::{thread, time::Duration};

#[derive(Deserialize, Serialize, Debug)]
pub struct Person {
    name: String,
    age: u8,
    score: Vec<u8>,
}

#[rpcfunc]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[rpcfunc]
pub fn who(p: Person) -> Vec<u8> {
    println!(
        "I am {}, {} years old, and my grade is {:?}",
        p.name, p.age, p.score
    );
    return p.score.clone();
}

fn main() {
    rpcserver::register("add".into(), add).unwrap();
    rpcserver::register("who".into(), who).unwrap();

    let addr = "127.0.0.1:5003".to_string();
    let cli = rpcclient::RpcClient::new(addr.clone());
    thread::spawn(move || rpcserver::run(&addr, |e| println!("{}", e.to_string())).unwrap());
    thread::sleep(Duration::from_millis(500));
    let bsum = callrpc!(fname "add".to_string(), client &cli, 1, 2).unwrap();
    let sum: i32 = rpcclient::deserialize(&bsum).unwrap();

    println!("got sum: {}", sum);

    let p = Person {
        age: 29,
        name: "Jack".into(),
        score: vec![1, 2, 3],
    };
    let bscore = callrpc!(fname "who", client &cli, p).unwrap();
    let score: Vec<u8> = rpcclient::deserialize(&bscore).unwrap();
    println!("got score: {:?}", score);
}
