/* L2 - Toolchain - Example 1
use std::io;

fn main() -> io::Result<()> {
    // print to stdout
    println!("Hello");

    //print to stderr
    eprintln!("world!");

    //Wait for enter key press
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

//* L2 - Toolchain - Example 3
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
 //*/

