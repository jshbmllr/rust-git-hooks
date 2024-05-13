use std::env;
use std::io::{stdin, BufRead};

fn main() {
    log();
}

pub fn log() {
    let name_arg = env::args().nth(0).unwrap_or(String::from("unknown"));
    let args: Vec<_> = env::args().skip(1).collect();
    println!("{} called with {:?}", name_arg, args);

    println!("BEGIN STDIN");
    let stdin = stdin();
    for line in stdin.lock().lines() {
        println!("{:?}", line);
    }
    println!("END STDIN");
}