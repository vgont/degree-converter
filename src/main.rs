use core::panic;
use std::io;

fn main() {
    println!("Choose what do you want to convert:\n");
    println!("1 - Fahrenheit to Celsius");
    println!("2 - Celsius to Fahrenheit");
    println!("\nType the option followed by the degree");
    println!("Example: 1 28");

    let mut input = String::new();
    let mut input_arguments;

    match io::stdin().read_line(&mut input) {
        Ok(_) => input_arguments = input.split_whitespace(),

        Err(_) => panic!("an error ocurred while trying to read the input"),
    }

    let chosen_option: u8 = input_arguments.next().unwrap().parse().unwrap();
    let degrees: f64 = input_arguments.next().unwrap().parse().unwrap();

    match &chosen_option {
        1 => {
            println!(
                "{}°F -> {}°C",
                degrees,
                convert_fahrenheit_to_celsius(&degrees)
            )
        }
        2 => {
            println!(
                "{}°C -> {}°F",
                degrees,
                convert_celsius_to_fahrenheit(&degrees)
            )
        }
        _ => panic!("Invalid option"),
    };
}

fn convert_fahrenheit_to_celsius(f: &f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

fn convert_celsius_to_fahrenheit(c: &f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}
