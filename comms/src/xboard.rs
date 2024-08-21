use crate::protocol::Protocol;

pub struct XBoard;

impl Protocol for XBoard {
    fn send(&self, message: &str) {
        println!("{}", message);
    }

    fn receive(&self) -> String {
        use std::io::{self, BufRead};
        let stdin = io::stdin();
        stdin.lock().lines().next().unwrap().unwrap()
    }
}
