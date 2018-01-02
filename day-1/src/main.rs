use std::io::{Read, self};

fn main() {
    let mut buffer = String::new();

    // Get the input
    io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let buffer = buffer.trim();

    // Convert input into a list of numbers
    let nums : Vec<u32> = buffer.chars().map(|c| {
        c.to_string().parse().unwrap()
    }).collect();

    let result = day1a(&nums);
    println!("One: {}", result);
    let result = day1b(&nums);
    println!("Two: {}", result);
}

fn day1a(input : &[u32]) -> u32 {
    let mut sum = 0;

    for (i, val) in input.iter().enumerate() {

        let prev = if i == 0 {
            input[input.len() - 1]
        } else {
            input[i - 1]
        };

        if *val == prev {
            sum += val
        }
    }

    sum
}

fn day1b(input : &[u32]) -> u32 {
    let mut sum = 0;

    for (i, val) in input.iter().enumerate() {
        let other = (i + input.len() / 2) % input.len();

        let prev = input[other];

        if *val == prev {
            sum += val
        }
    }

    sum
}
