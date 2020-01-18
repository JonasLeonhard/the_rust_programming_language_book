use std::io;

#[derive(Debug)]
pub enum Temperature {
    Celcius(f32),
    Fahrenheit(f32),
}

/// ### User input
/// get the user input.
/// 01. read input into buffer via stdin()
/// ```
/// println!("\nPlease input a temperature you want to convert (Format: 100F or -40C):");
///        let mut buf = String::new();
///        io::stdin()
///        .read_line(&mut buf)
///        .expect("Error: Failed input while reading line.");
/// ```
/// 02. trim input leading / trailing whitespaces
/// ```
/// let trimmed = buf.trim();
/// ```
/// 03. quit loop on input == "quit"
/// 04. parse temp to <f32>
/// let temp = match temp.parse::<f32>() {
///            Ok(num) => num,
///            Err(_) => {
///                println!("Error: Failed parsing input buffer to <f32>. Input must be a number:");
///                continue; // starts the loop over
///            },
///        };
/// 05 create Temperature:: Fahrenheit | Celcius from scale
/// ```
/// let temperature= match scale
/// {
///           "C" | "c" => Temperature::Celcius(temp),
///            "F" | "f" => Temperature::Fahrenheit(temp),
///            _ => {
///                println!("Error: failed matching scale. Try {{Format: 60F or -20C}}:");
///                continue;
///            },
///        };
/// ```
pub fn get_user_input() {
    println!("\nType \"quit\" to end the program");

    loop {
        // read user input
        println!("\nPlease input a temperature you want to convert (Format: 100F or -40C):");
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Error: Failed input while reading line.");

        // trim of leading / trailing whitespaces
        let trimmed = buf.trim();

        if trimmed == "quit" {
            break;
        }

        // split string into tuple at last character
        let (temp, scale) = trimmed.split_at(trimmed.len() - 1);

        // check if temp is a number
        let temp = match temp.parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: Failed parsing input buffer to <f32>. Input must be a number:");
                continue; // starts the loop over
            }
        };

        // check for type and create a Temperature Enum with #(temp) inside
        let temperature = match scale {
            "C" | "c" => Temperature::Celcius(temp),
            "F" | "f" => Temperature::Fahrenheit(temp),
            _ => {
                println!("Error: failed matching scale. Try {{Format: 60F or -20C}}:");
                continue;
            }
        };

        print_temp(&temperature);
    }
}
/// #### Doc
/// prints out 10 smaple temperatures with their converted Temperature (F/C)
///
/// #### Examples
///
/// ```
/// sample_temperature();
/// ```
pub fn sample_temperature() {
    let temperatures = [
        Temperature::Fahrenheit(50.0),
        Temperature::Fahrenheit(70.0),
        Temperature::Fahrenheit(100.0),
        Temperature::Fahrenheit(150.0),
        Temperature::Fahrenheit(200.0),
        Temperature::Celcius(3.0),
        Temperature::Celcius(5.0),
        Temperature::Celcius(10.0),
        Temperature::Celcius(50.0),
        Temperature::Celcius(100.0),
    ];

    for temp in &temperatures {
        // or .iter() to mutate obj
        print_temp(temp);
    }
}

pub fn print_temp(temp: &Temperature) {
    match temp {
        Temperature::Celcius(deg) => {
            println!("Celcius: {:?} => Fahrenheit {:?}", deg, convert_temp(temp));
        }
        Temperature::Fahrenheit(deg) => {
            println!("Fahrenheit: {:?} => Celcius {:?}", deg, convert_temp(temp));
        }
    }
}

pub fn convert_temp(temp: &Temperature) -> f32 {
    match temp {
        Temperature::Celcius(val) => (val * 1.8) + 32.0,
        Temperature::Fahrenheit(val) => (val - 32.0) / 1.8,
    }
}
