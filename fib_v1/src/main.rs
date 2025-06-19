#![allow(dead_code)]

use std::io;
use num_bigint::BigUint;
use std::collections::HashMap;

fn main() {
    loop {
        println!("Please type in the position of the Fibonacci number you want to know:");
        let mut user_input = String::new();
        let _input_result = io::stdin().read_line(&mut user_input);

        // type check
        match user_input.trim().parse::<u128>() {
            Ok(position) => {
                println!("The {position}-th Fibonacci number is {}.", big_fib(position));
            }
            Err(_) => {
                // nothing => next iteration
            }
        }
    }
}

fn iterator_fib(n:u32) -> u64 {
    if n == 0 || n == 1{
        return 1;
    }
    let mut map = HashMap::new();
    map.insert(0u32, 1u32);
    map.insert(1u32,1u32);
    for i in 2..=n {
        map.insert(i, *map.get(&(i - 1)).unwrap() + *map.get(&(i - 2)).unwrap());
    }
    *map.get(&n).unwrap() as u64
}

fn big_fib(n:u128) -> BigUint {
    let mut prev = BigUint::from(0u8);
    let mut now = BigUint::from(1u8);
    for _ in 0..n{
        let next =  prev + &now;
        prev = now;
        now = next;
    }
    prev
}

fn iterative_fib(n: u32) -> u64 {
    let mut prev: u64 = 0;
    let mut now: u64 = 1;
    for _ in 0..n{
        let next =  prev + now;
        prev = now;
        now = next;
    }
    prev
}

fn _recursive_fib(n: u16) -> u32 {
    if n == 0 {
        0
    } else if n == 1 || n == 2 {
        1
    } else {
        _recursive_fib(n-1) + _recursive_fib(n-2)
    }
}