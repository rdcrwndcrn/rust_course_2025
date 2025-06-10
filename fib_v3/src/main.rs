use num_bigint::BigUint;
use std::io;

#[derive(Debug, Clone)]
struct Fibonacci {
    a: BigUint,
    b: BigUint,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            a:BigUint::from(0u32),
            b:BigUint::from(1u32),
        }
    }
}

impl Iterator for Fibonacci {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item>{
        let current = self.a.clone();
        let c = &self.a + &self.b;
        self.a = self.b.clone();
        self.b = c.clone();
        Some(current)
    }

}

fn main() {
    let mut fib = Fibonacci::new();

    println!("Enter number to calculate the n-th Fibonacci for n =");
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    let number = buffer
        .trim()
        .parse::<BigUint>()
        .unwrap_or(BigUint::from(10_u32));

    // For the loop, we need to make use of a special method offered by Iterator trait
    let mut iteration = BigUint::from(0u8);
    let one = BigUint::from(1u8);
    while &iteration < &number {
        let x = fib.next().unwrap();
        println!("{x}");
        iteration += &one;
    }
    /*
    let number = buffer.trim().parse::<usize>().unwrap_or(10);

    // For the loop, we need to make use of a special method offered by Iterator trait
    for value in fib.take(number) {
        println!("{value} ");
    }
     */
}
