
use std::io;

fn main() {
    let a: [i32;5] = [1, 2, 3, 4, 5];

    println!("Enter a number ");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Failed to convert into number");

    let element = a[index];

    println!("The value of {} is {} ", index, element);
}
