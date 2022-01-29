
use std::io;

enum Options {
    ConvertTemperature,
    Fibonacci,
    Song,
    Invalid
}

fn main() {
    println!("**** EXERCISES *****");

    loop {
        println!("Enter any option (1, 2, 3, 4)");
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Error reading input");
        let option: i32 = option.trim().parse().expect("Error conversion");
    
        let enum_option = match option {
            1 => Options::ConvertTemperature,
            2 => Options::Fibonacci,
            3 => Options::Song,
            _ => Options::Invalid,
        };

        match enum_option {
            Options::ConvertTemperature => convert_temperature(),
            Options::Fibonacci => fib_entry(),
            Options::Song => print_song(),
            Options::Invalid => {
                println!("Quitting the program");
                break;
            },
        }
    }
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn fib_entry() {
    println!(" *** Fibonacci calculation *** ");
    println!(" Enter the number to find fibonacci ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    let input: u64 = input.trim().parse().expect("Error converting to int");
    println!("Result is : {}", fibonacci(input));
}

fn convert_temperature() {
    println!(" *** Temperature conversion *** ");
    println!(" Enter the temperature ");

    // get input
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Error reading input");
    let temp: f32 = temp.trim().parse().expect("Error converting to float");

    println!(" Enter the temperature format ");

    let mut unit = String::new();
    io::stdin().read_line(&mut unit).expect("Error reading unit");
    let unit: char = unit.trim().parse().expect("Error converting to char");

    let converted_unit: char = if unit == 'F' || unit == 'f' { 'C' } else { 'F' };
    let converted_val: f32 = if converted_unit == 'C' { ftoc(temp) } else if converted_unit == 'F' { ctof(temp) } else { 0.0 };

    println!(" Converted value: {:.2} in {}", converted_val, converted_unit);
}

// receives in farenheit and returns celcius
fn ftoc(f:f32) -> f32 {
    (f - 32 as f32) * 5 as f32/9 as f32
}

// receives in celcius and returns farenheit
fn ctof(c:f32) -> f32 {
    (c * 9.0/5.0) + 32.0
}

fn print_song() {
    for element in 1..=5 {
        let day_string = match element {
            1 => "First",
            2 => "Second",
            3 => "Third",
            4 => "Fourth",
            5 => "Fifth",
            _ => "last",
        };
        println!("{} day of christmas", day_string);
    }
}
