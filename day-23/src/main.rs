mod register;

use std::io::Read;

fn main() {
    let mut buffer = String::new();

    // Get the input
    std::io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let buffer = buffer.trim();

    let lines : Vec<&str> = buffer.split('\n').collect();
    let input : Vec<register::Instruction> = lines.iter()
        .map(|line| {
            register::Instruction::parse(line)
        }).collect();

    let result = day23a(&input);
    println!("One: {}", result);
}

fn day23a(input: &[register::Instruction]) -> usize {
    let mut coprocessor = register::Coprocessor::new(&input);

    while !coprocessor.is_done() {
        coprocessor.step();
    }

    coprocessor.multiplications()
}
