use std::{env, io};

fn main() {
    // Supports both command line input and user input.
    let args: Vec<String> = env::args().collect();

    let mut input= String::new();

    if args.len() > 1 {
        input = args[1].clone();
    } else {
        println!("Generate Fibonacci Numbers!");
        println!("Please enter how many to generate:");

        io::stdin()
            .read_line(&mut input)
            .expect("TODO: panic message");
    }

    let input: u32 = input.trim().parse().expect("Please enter a number.");

    let mut first: i128 = 1;
    let mut second: i128 = 1;
    let mut next: i128;

    for _i in 1..=input {
        println!("{first} ");
        next = first + second;
        first = second;
        second = next;
    }
}
