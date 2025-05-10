use std::io;

fn main() {
    loop{
        //ask for the unit which we convert from
        println!("Which unit do you want to convert from? Press: Press: \n'1' for Celsius, \n'2' for Fahrenheit, \n'3' for Kelvin and \n'q' for quitting");
        let from = get_user_input() as u8;

        // ask which conversion is wanted, get user input and save it
        println!("Which unit do you want to convert to? Press: \n'1' for Celsius, \n'2' for Fahrenheit, \n'3' for Kelvin.");
        let to = get_user_input() as u8;

        println!("Put you number in:");
        let number = get_user_input();

        match (from,to) {
            (1,1) => {
                println!("{number}°C are {number}°C");
            },
            (1,2) => {
                println!("{number}°C are {}°F",celsius_to_fahrenheit(number));
            },
            (1,3) => {
                println!("{number}°C are {}°K", celsius_to_kelvin(number));
            },
            (2,1) => {
                println!("{number}°F are {}°C", fahrenheit_to_celsius(number));
            },
            (2,2) => {
                println!("{number}°F are {number}°F");
            },
            (2,3) => {
                println!("{number}°F are {}°K", fahrenheit_to_kelvin(number));
            },
            (3,1) => {
                println!("{number}°K are {}°C", kelvin_to_celsius(number));
            },
            (3,2) => {
                println!("{number}°K are {}°F", kelvin_to_fahrenheit(number));
            },
            (3,3) => {
                println!("{number}°K are {number}°K");
            }
            (_,_) => {
                continue;
            }
        }
    }
}

fn celsius_to_fahrenheit(value: f64) -> f64 {
    (value * 1.8) + 32.0
}

fn celsius_to_kelvin(value: f64) -> f64 {
    value + 273.15
}

fn fahrenheit_to_celsius(value: f64) -> f64 {
    (value - 32.0) / 1.8
}

fn fahrenheit_to_kelvin(value: f64) -> f64 {
    ((value - 32.0) / 1.8) + 273.15
}

fn kelvin_to_celsius(value: f64) -> f64 {
    value - 273.15
}

fn kelvin_to_fahrenheit(value: f64) -> f64 {
    ((value - 273.15) / 1.8) + 32.0
}

fn get_user_input() -> f64 {
    loop {
        let mut user_input = String::new();
        let read_result = io::stdin().read_line(&mut user_input);
        // test if input is actionable
        match read_result {
            Ok(_) => {
                // trim useless stuff from the input for easier further inspection
                let user_input_trimmed = user_input.trim();
                // check if the user wants to leave
                if user_input_trimmed == "q" {
                    println!("Until next time!");
                    // ending program
                    std::process::exit(0);
                }
                // check if input is usable and if that's the case, return it
                match user_input_trimmed.parse::<f64>() {
                    Ok(value) => {
                        // returning value
                        return value;
                    }
                    Err(_) => {
                        println!("Invalid input, again:");
                    }
                }

            }
            // invalid input starts the loop over
            _ => {
                println!("Invalid input, again:");
            }
        }
    }
}