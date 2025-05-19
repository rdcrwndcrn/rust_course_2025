/* L2 - Toolchain - Example 1
use std::io;

fn main() -> io::Result<()> {
    // print to stdout
    println!("Hello");

    //print to stderr
    eprintln!("world!");

    //Wait for the enter key press
    println!("Press enter to continue...");
    let mut _buffer = String::new();
    io::stdin().read_line(&mut _buffer)?;

    Ok(())
}
*/

/* L2 - Toolchain - Example 2
use std::io;

fn read_prompt(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    input
}
fn main() {
    // Print to stdout+
    println!("Hello");

    // print to stderr
    eprintln!("World");

    // Wait for enter key to press
    read_prompt("Press enter to continue");
}
*/

/* L2 - Toolchain - Example 3
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
    loop{
        // Generate random 32-character String
        let random_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            . take(32)
            .map(char::from)
            .collect();

        // Print the String
        println!("Random string: {}", random_string);

        // Wait for 10 seconds
        thread::sleep(Duration::from_secs(10));
    }
}
*/

/* L3 - Toolchain - Example 1
// understanding implicit and explicit positional arguments for formating strings

fn main() {
    println!("{} , {2} , {1} , {} , {0}", 1, 2, 3);           
    // println!("{3} , {2} , {1}", 1, 2, 3);                     // index 3 does not exist
    // println!("{3} , {2} , {1}", 1, 2, 3, 4);                  // argument 1 never used
    println!("{} , {3} , {2} , {1}", 1, 2, 3, 4);              
    // println!("{} , {2} , {1} , {0}", 1, 2, 3, 4);             // argument 4 never used

}
 */

/* L3 - Toolchain - Example 2
// pause vio stdin
use std::io;

fn main() -> io::Result<()> {
    // Print to stdout
    println!("Hello");  // Will be printed to stdout

    // Print to stderr
    eprintln!("World"); // Will be printed to stderr

    // Wait for the Enter key press
    println!("Press Enter to continue...");
    let mut _buffer = String::new();
    io::stdin().read_line(&mut _buffer)?;

    Ok(())
}
*/

// L3 - Toolchain - Example 3
/* Quine
fn main() {
    print!(
        "fn main(){{print!({0:?},{0:?})}}",
        "fn main(){{print!({0:?},{0:?})}}"
    )
}
 */


// L3 - Toolchain - Example 4
/* Quine with raw strings and without debug print
fn main(){print!(r#"fn main(){{print!(r#{1}{0}{1}#,r#{1}{0}{1}#,'"')}}"#,r#"fn main(){{print!(r#{1}{0}{1}#,r#{1}{0}{1}#,'"')}}"#,'"')}
 */

/* L3 - Toolchain - Example 5
// generate a sequence of random numbers
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        // Generate a random 32-character string
        let random_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        // Print the string
        println!("{}", random_string);

        // Wait for 10 seconds
        thread::sleep(Duration::from_secs(2));
    }
}
 */

/* L4 - Toolchain - Example 1

fn main() {
    let é = 0xE9; // variable names can contain unicode characters
    let ö = 0xF6;
    let é_utf_8 = "é".as_bytes(); // &[u8]
    let ö_utf_8 = "ö".as_bytes(); // &[u8]
    println!(
        "é: unicode {é:b} = utf-8 {0:x},{1:x} (hex) = {0:b},{1:b} (binary)",
        é_utf_8[0],
        é_utf_8[1]
    );
    //noinspection RsInvalidFormatString
    println!(
        "ö: unicode {ö:b} = utf-8 {0:x},{1:x} (hex) = {0:b},{1:b} (binary)",
        ö_utf_8[0],
        ö_utf_8[1]
    );
}
 */

/* L4 - Toolchain - Example 2

fn main() {
    println!("123456781234567812345678");
    println!("Aa        B    C");
    println!("Aa\tB\tC");
    println!("\tB\tC");
    println!("Aa\rB\tC");
    println!("Aa\rB    C");
}
*/

/* L4 - Toolchain - Example 3
fn main() {
    println!("    println!(\"123456781234567812345678\")");
    println!("    println!(\"Aa        B        C\")");
    println!("    println!(\"Aa\\tB\\tC\")");
    println!("    println!(\"\\tB\\tC\")");
    println!("    println!(\"Aa\\rB\\tC\")");
    println!("    println!(\"Aa\\rB        C\")");
    println!("Prints:");
}
 */

/* L4 - Toolchain - Example 4

fn main() {
    // A byte string literal — an &[u8] of the ASCII codes for "Hello"
    let bytes = b"Hello";
    // The same bytes, written with hexadecimal escapes
    let hex   = b"\x48\x65\x6C\x6C\x6F";

    // Debug-print them as byte arrays
    println!("bytes = {:?}", bytes); // bytes = [72, 101, 108, 108, 111]
    println!("hex   = {:?}", hex);   // hex   = [72, 101, 108, 108, 111]

    // Verify they’re equal
    assert_eq!(bytes, hex);
}
 */

/* EV4 - Example 1
#[derive(Debug)]
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500,
}
fn main() {
    let a = HttpStatus::Ok;
    let b = HttpStatus::NotFound;
    let c = HttpStatus::InternalServerError;
    println!("Ok: {:#?}", a );
    println!("NotFound: {}", b as u16);
    println!("InternalServerError: {}", c as u16);
}
*/

/* EV4 - Example 2
fn slicer(input: &str) -> Option<usize> {
    // Iterate through all chars in input:
    for (i, c) in input.chars().enumerate() {
        if c == ' ' {
            // Return index of whitespace:
            return Some(i);
        }
    }

    // If input doesn't contain whitespace:
    None
}

fn main() {
    let word = String::from("Please separate me ignoring  double whitespaces");

    // Create mutable '&str' from 'String':
    let mut new_word = &word[..]; // The magic of omitting

    loop {
        let idx = slicer(new_word);
        // println!("IDX: {:?}", idx);
        match idx {
            Some(idx) => {
                if idx > 0 { // We ignore double whitespaces
                    println!("Sliced word: {}", &new_word[..idx]); // Slice from the start
                } else {
                    // println!("Whitespace found!");
                }
                new_word = &new_word[(idx + 1)..]; // Re-slice until the end
            }
            None => {
                println!("Sliced word: {}", new_word);
                break;
            }
        };
    }
}
 */

/* EV4 - Example 3
fn print_data(data: Option<&char>) {
    match data {
        Some(data) => println!("Found character: {data}"),
        None => println!("No character found."),
    }
}
fn main() {
    let v1 = vec!['R', 'u', 's', 't'];
    let elem: Option<&char> = v1.get(2);
    print_data(elem);
    print_data(v1.get(4));
}
 */

fn main() {
    {
        for c in "Ле".chars() {
            println!("{c}");
        }

        for b in "Ле".bytes() {
            println!("{b}");
        }
    }
}
