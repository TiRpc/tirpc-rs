use bincode;
use macros::rpcfunc;
use rpcclient::{self, callrpc};
use serde::{Deserialize, Serialize};
use std::{thread, time::Duration};
use tuplecaller::TupleCaller;

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
    rpcserver::register("add".into(), add);
    rpcserver::register("who".into(), who);

    let addr = "127.0.0.1:5003".to_string();
    let cli = rpcclient::RpcClient::new(addr.clone());
    thread::spawn(move || rpcserver::run(&addr));
    thread::sleep(Duration::from_millis(500));
    let bsum = callrpc!(fname "add".to_string(), client &cli, 1, 2);
    let sum: i32 = rpcclient::deserialize(&bsum);

    println!("got sum: {}", sum);

    let p = Person {
        age: 29,
        name: "Jack".into(),
        score: vec![1, 2, 3],
    };
    let bscore = callrpc!(fname "who", client &cli, p);
    let score: Vec<u8> = rpcclient::deserialize(&bscore);
    println!("got score: {:?}", score);
}
