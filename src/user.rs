use std::io;
use std::io::prelude::*;

pub fn get_loop_range() -> u64 {
    let mut number = String::new();

    let num: u64;

    print!("Digite o loop: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut number).unwrap();
    num = number.trim().parse::<u64>().unwrap();
    num
}
