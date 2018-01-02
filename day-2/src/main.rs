use std::io::{Read, self};

fn main() {
    let mut buffer = String::new();

    // Get the input
    io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let buffer = buffer.trim();

    let lines : Vec<&str> = buffer.split('\n').collect();
    let input : Vec<Vec<u32>> = lines.iter().map(|row| {
        row.split_whitespace().map(|v| v.parse().unwrap()).collect()
    }).collect();

    let result = day2a(&input);
    println!("One: {}", result);
    let result = day2b(&input);
    println!("Two: {}", result);
}

fn day2a(input : &[Vec<u32>]) -> u32 {
    input.iter().map(|row| {
        let mut smallest : u32 = 9999999;
        let mut largest : u32 = 0;

        for i in row {
            if *i > largest {
                largest = *i;
            }
            if *i < smallest {
                smallest = *i;
            }
        }

        largest - smallest
    }).sum()
}

fn day2b(input : &[Vec<u32>]) -> u32 {
    input.iter().map(|row| {
        for (i, ival) in row.iter().enumerate() {
            for (j, jval) in row.iter().enumerate() {
                if i == j {
                    continue;
                }
                if ival < jval {
                    continue;
                }

                if ival % jval == 0 {
                    return ival / jval;
                }
            }
        }

        0
    }).sum()
}
