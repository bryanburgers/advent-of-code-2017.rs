use std::io::{Read, self};

#[derive(Debug)]
struct Step {
    index : i32,
    data : Vec<i32>,
}

impl Step {
    fn new(data : &[i32]) -> Step {
        let data : Vec<i32> = data.iter().map(|i| *i).collect();
        Step { index: 0, data: data }
    }

    fn done(&self) -> bool {
        self.index < 0 || self.index >= (self.data.len() as i32)
    }

    fn next(&mut self) {
        if self.done() {
            panic!("Already done!");
        }

        let current_val = self.data[self.index as usize];
        self.data[self.index as usize] = current_val + 1;
        self.index += current_val;
    }

    fn next2(&mut self) {
        if self.done() {
            panic!("Already done!");
        }

        let current_val = self.data[self.index as usize];
        let change = if current_val >= 3 {
            current_val - 1
        } else {
            current_val + 1
        };
        self.data[self.index as usize] = change;
        self.index += current_val;
    }
}

fn main() {
    let mut buffer = String::new();

    // Get the input
    io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let buffer = buffer.trim();

    let input : Vec<i32> = buffer.split('\n').map(|line| line.parse().unwrap()).collect();

    let result = day5a(&input);
    println!("One: {}", result);
    let result = day5b(&input);
    println!("Two: {}", result);
}

fn day5a(input : &[i32]) -> u32 {
    let mut step = Step::new(input);
    let mut count = 0;

    while !step.done() {
        count += 1;
        step.next();
    }

    count
}

fn day5b(input : &[i32]) -> u32 {
    let mut step = Step::new(input);
    let mut count = 0;

    while !step.done() {
        count += 1;
        step.next2();
    }

    count
}
