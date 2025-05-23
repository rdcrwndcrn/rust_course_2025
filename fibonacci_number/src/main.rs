use std::io;
use num_bigint::BigUint;

fn main() {
    loop {
        println!("Please type in the position of the Fibonacci number you want to know:");
        let mut user_input = String::new();
        let _input_result = io::stdin().read_line(&mut user_input);

        // type check
        match user_input.trim().parse::<u128>() {
            Ok(position) => {
                println!("The {position}-th Fibonacci number is {}.", big_fibonacci(position));
            }
            Err(_) => {
                // try again
            }
        }
    }
}

fn iterative_fibonacci(n: u32) -> u64 {
    let mut prev: u64 = 0;
    let mut now: u64 = 1;
    for _ in 0..n{
        let next =  prev + now;
        prev = now;
        now = next;
    }
    prev
}

fn _recursive_fibonacci(n: u16) -> u32 {
    if n == 0 {
        0
    } else if n == 1 || n == 2 {
        1
    } else {
        _recursive_fibonacci(n-1) + _recursive_fibonacci(n-2)
    }
}

fn big_fibonacci(n:u128) -> BigUint {
    let mut prev = BigUint::from(0u8);
    let mut now = BigUint::from(1u8);
    for _ in 0..n{
        let next =  prev + &now;
        prev = now;
        now = next;
    }
    prev
}

