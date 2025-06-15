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


// 4. Task: Quine with raw strings
fn main() {
    print!(r##"fn main(){{print!(r#"{0}"#,r#"{0}"#)}}"##,
           r##"fn main(){{print!(r#"{0}"#,r#"{0}"#)}}"##
    )
}