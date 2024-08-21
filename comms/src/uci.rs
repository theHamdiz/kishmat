use crate::protocol::Protocol;

pub struct Uci;

impl Protocol for Uci {
    fn send(&self, message: &str) {
        println!("{}", message);
    }

    fn receive(&self) -> String {
        use std::io::{self, BufRead};
        let stdin = io::stdin();
        stdin.lock().lines().next().unwrap().unwrap()
    }
}
