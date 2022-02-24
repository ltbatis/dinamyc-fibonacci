use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn main() {
    let num = get_loop_range();
    loop_dyn_fib(num)
}

fn get_loop_range() -> u64 {
    let mut number = String::new();

    let num: u64;

    print!("Digite o loop: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut number).unwrap();
    num = number.trim().parse::<u64>().unwrap();
    num
}

fn loop_dyn_fib(num: u64) {
    let mut map = HashMap::new();
    for i in 1..num {
        println!("{}: {}", i, dynamic_fib(i, &mut map))
    }
}

fn dynamic_fib(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    match n {
        0 | 1 => 1,
        n => {
            if map.contains_key(&n) {
                *map.get(&n).unwrap()
            } else {
                let val = dynamic_fib(n - 1, map) + dynamic_fib(n - 2, map);
                map.insert(n, val);
                val
            }
        }
    }
}
