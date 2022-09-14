use std::env;
use std::process::exit;
use std::num::ParseFloatError;

mod temp_converter;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("you must supply a number and a unit");
        exit(1);
    }

    let number_arg: Result<f32, ParseFloatError> = args[1].parse();
    let unit: char = args[2].parse().unwrap();

    let number: f32;
    match number_arg {
        Ok(num) => {
            number = num;
        }

        Err(_) => {
            println!("not a valid number");
            exit(1);
        }
    }

    let convertion: f32;
    match unit {
        'c' => {
            convertion = temp_converter::celsius_to_fahrenheit(number as f32);
        }
        'f' => {
            convertion = temp_converter::fahrenheit_to_celsius(number as f32);
        }
        _ => {
            println!("invalid unit");
            exit(1);
        }
    }
    println!("{}", convertion);
}
