// Exercise 2 Tasks:

/* 2. Task: Copy vs. Borrow
fn main() {
    let x: i32 = 5;

    println!("x: {:p}", &x);

    borrow(&x);
    copy(x);
}

// This function should borrow "x"
fn borrow(y: &i32) {
    println!("y: {:p}", y);
}

// This method should copy "x"
fn copy(z: i32) {
    println!("z: {:p}", &z);
}
 */

// 3. Task: Slices and Ranges
/* a) char range and ASCII conversion
fn main() {
    for i in 'a'..='z' {
        println!("{}", i.to_uppercase());
    }
}
*/
//b) Text analyzer
// Write a small program that reads a text from the command line (e.g.: stdin),
// creates a slice for each word and print the number of letters per word. 
use std::io;

fn main() {
    let input = get_input();
    
    let mut last = 0;
    let mut now;
    loop {
        now = slicer(&input[last..]);
        let slice = &input[last..last + now].trim();
        last += now;
        if now == 0 {
            break;
        }
        println!("slice: {}, length: {}, nouns: {}", slice, count_chars(slice), count_nouns(slice));
    }
}

fn get_input() -> String  {
    let mut input = String::new();
    let read_result = io::stdin().read_line(&mut input);
    match read_result { 
        Ok(_) => input.trim().to_string(),
        Err(_) => get_input()
    }
}

fn slicer(slice: &str) -> usize {
    for (i, c) in slice.chars().enumerate() {
        if  c == ' ' {
            return i + 1;
        }
    }
    slice.len()
}

fn count_chars (slice: &str) -> usize {
    slice.trim().len()
}

fn count_nouns (slice: &str) -> usize {
    let mut count = 0;
    for c in slice.chars() {
        if ['a', 'e', 'i', 'o', 'u', 'ö', 'ä', 'ü'].contains(&c){
            count += 1;
        }
    }
    count
}
