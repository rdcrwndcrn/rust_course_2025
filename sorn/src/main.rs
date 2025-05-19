use clap::Parser;

fn generate_random_string(length: usize) -> String {
    use rand::Rng;
    use rand::distr::Alphanumeric;

    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn do_sleep(seconds: u64) {
    use std::thread;
    use std::time::Duration;

    thread::sleep(Duration::from_secs(seconds));
}

#[derive(Parser, Debug)]
/// sorn is a CLI tool to generate <REPEAT> random strings of length <LENGTH> and sleeps for <SLEEP> seconds between outputs.
///
/// The program is designed to be run from the command line and takes the following arguments:
///
/// `--length`: The length of the random string to generate.
///
/// `--sleep`: The number of seconds to sleep between generating random strings.
///
/// `--repeat`: The number of times to repeat the process of generating a random string and sleeping.
///
/// The program uses the `clap` crate for command line argument parsing.
/// The program uses the `rand` crate to generate random strings and the `std::thread` module to sleep.
struct Args {
    /// number of random characters to generate
    #[clap(short, long, default_value = "6")]
    length: usize,
    /// count of components per line
    #[clap(short, long, default_value = "3")]
    count: usize,
    /// number of seconds to sleep
    #[clap(short, long, default_value = "1")]
    sleep: u64,
    /// number of times to repeat
    #[clap(short, long, default_value = "1")]
    repeat: u64,
    /// ap- and prepend "
    #[clap(long, default_value = "true")]
    raw: bool,
}

fn main() {
    let args = Args::parse();
    let length = args.length;
    let count = args.count;
    let sleep = args.sleep;
    let repeat = args.repeat;
    let raw = args.raw;
    for i in 0..repeat {
        let mut result = String::new();
        // check if there are multiple components and
        if count == 0 {
            break;
        } else if count == 1 {
            // add only component
            let random_string = generate_random_string(length);
            result = format!("{random_string}");
        } else {
            // print all components to one line except the last
            for _ in 1.. count {
                let random_string = generate_random_string(length);
                result = format!("{result}{random_string}-");
            }
            // print the last components with "
            let random_string = generate_random_string(length);
            result = format!("{result}{random_string}");
        }
        if raw {
            result = format!("\"{result}\"");
        }
        println!("{result}");
        // check if we are at the last iteration
        if i + 1 == repeat {
            // skip the unnecessary sleep
            break;
        }
        do_sleep(sleep);
    }
}
