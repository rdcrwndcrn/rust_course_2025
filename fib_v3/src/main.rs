use num_bigint::BigUint;
use std::io;

#[derive(Debug, Clone)]
struct Fibonacci {
    a: BigUint,
    b: BigUint,
}

impl Iterator for Fibonacci {
    // @TODO Add correct type for alias "Item" and implement the method "next()"
    type Item = BigUint;

}

fn main() {
    let fib = Fibonacci {
        a: BigUint::from(0_u32),
        b: BigUint::from(1_u32),
    };

    println!("Enter number to calculate Fibonacci for:");
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    let number = buffer
        .trim()
        .parse::<BigUint>()
        .unwrap_or(BigUint::from(10_u32));

    // For the loop, we need to make use of a special method offered by Iterator trait
    // @TODO Implement the loop below
}
