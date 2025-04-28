// Exercise 1 lecture examples:

/* type boundaries
fn main() {
    println!("f32 boundaries {} and {}", f32::MIN, f32::MAX);
}
*/

/* getting user input
use std::io;
fn main() {
    let mut user_input = String::new();
    let read_result = io::stdin().read_line(&mut user_input);
    println!("User input is: {}", user_input);
    println!("User input read result: {:?}", read_result);
}
*/

/* passing user input as argument
use std::env;
fn main() {
    for arg in env::args() {
        println!("{}", arg);
    }
}
*/

/*
use std::env;
fn main() {
    if let Ok(path) = env::var("PATH") {
        println!("Dein PATH ist: {}", path);
    } else {
        println!("PATH nicht gesetzt!");
    }
}
*/



// Exercise 1 Tasks:

// 2. Task: Operator
/* a) Shift Operators:
// Write a small program that defines two integer values and shifts the first value to the left (<<) by n positions (the second value).
// Print the final result.
fn shift_left(mut number: i32, positions: u8) -> () {
    println!("The value of x is: {number}, in binary:{:0b}", number);
    number = number << positions;
    println!("after shifting {positions} the value is {number}, in binary:{:0b}", number);
}

fn shift_right(mut number: i32, positions: u8) -> () {
    println!("The value of x is: {number}, in binary:{:0b}", number);
    number = number >> positions;
    println!("after shifting {positions} the value is {number}, in binary:{:0b}", number);
}
fn main() {
    shift_left(21, 3u8);
    shift_right(672, 2u8);
}
*/

/* b) Bitwise Operators:
// Write a small program that combines two integer values using bitwise operators.
// Try out all the operators (AND, OR, XOR, NOT) with different values and print the result.

fn bit_and(x:i32, y:i32) -> i32 {
    println!("We got: {x}, in binary {:b} and {y}, in binary {:b}", x, y);
    let z = x & y;
    println!("after and operator we get {z}, in binary {:b}", z);
    z
}

fn bit_or(x:i32, y:i32) -> i32 {
    println!("We got: {x}, in binary {:b} or {y}", x);
    let z = x | y;
    println!("after or operator we get {z}, in binary {:b}", z);
    z
}

fn bit_xor(x:i32, y:i32) -> i32 {
    println!("We got: {x}, in binary {:b} and {y}", x);
    let z = x ^ y;
    println!("after xor operator we get {z}, in binary {:b}", z);
    z
}

fn bit_not(x:i32) -> i32 {
    println!("We got: {x}, in binary {:b}", x);
    let z = !x;
    println!("after not, we get {z}, in binary {:b}", z);
    z
}

fn main() {
    bit_and(5,7);
    bit_or(5,7);
    bit_xor(5,7);
    bit_not(5);
}
*/

// Task 3:  Environment Variables
// a) env.args()
// Write a small program that reads a list of command line arguments passed with the execution.
// Process the parameters and print them.
/*
use std::env;
fn main() {
    for args in env::args(){
        println!("{}",args);
    }
}
*/

// b) env.var()
// Write a small program that reads a value from some pre-defined environment variable RUST_INPUT.
// Process the parameters and print them.
use std::env;
fn main() {
    for (key,value) in env::vars() {
        println!("{}: {}", key, value);
    }
}


/* test example
// added public function for test purposes
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]      // so it only gets attention when using cargo test
mod tests {       // module test, so test arent in the top_level of the file?
    use super::*; // so the test can use functions from the outer scope, where add_one is

    #[test]       //  marks the following functions as a test
    fn basic_test() {
        assert_eq!(3, add_one(2));
    }
}
*/