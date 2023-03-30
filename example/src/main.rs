use bincode;
use calltuple::CallTuple;
use macros::rpcfunc;
use serde::{Deserialize, Serialize};
use std::thread;
use client::{self, callrpc};
use std::time::Duration;

#[derive(Deserialize, Serialize, Debug)]
pub struct Person{
    name: String,
    age: u8,
    score: Vec<u8>,
}

#[rpcfunc]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}


#[rpcfunc]
pub fn who(p: Person) -> Vec<u8>{
    println!("I am {}, {} years old, and my grade is {:?}", p.name, p.age, p.score);
    return p.score.clone();
}


fn main() {
    server::register("add".into(), add);
    server::register("who".into(), who);

    let addr = "127.0.0.1:5003".to_string();
    let cli = client::RPCClient::new(addr.clone());
    thread::spawn(move||server::run(&addr));
    thread::sleep(Duration::from_millis(500));
    let vsum = callrpc!(fname "add".to_string(), client &cli, 1, 2);
    let sum:i32 = client::deserialize(&vsum);

    println!("got sum: {}", sum);

    let p = Person{
        age: 29,
        name: "Jack".into(),
        score: vec![1,2,3],
    };
    let bscore = callrpc!(fname "who", client &cli, p);
    let score: Vec<u8> = client::deserialize(&bscore);
    println!("got score: {:?}", score);
}
