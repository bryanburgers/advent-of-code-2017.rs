use std::io::{Read, self};
use std::collections::HashMap;
use std::fmt;

fn main() {
    let mut buffer = String::new();

    // Get the input
    io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let buffer = buffer.trim();

    let lines : Vec<&str> = buffer.split('\n').collect();
    let input : Vec<Instruction> = lines.iter().map(|line| {
        let mut parts_iter = line.split_whitespace();

        let modify_register = Register {
            name: String::from(parts_iter.next().unwrap())
        };
        let modify_direction = match parts_iter.next().unwrap() {
            "inc" => ChangeDirection::Inc,
            "dec" => ChangeDirection::Dec,
            _ => panic!("Unknown direction"),
        };
        let modify_amount = parts_iter.next().unwrap().parse().unwrap();

        // if
        parts_iter.next().unwrap();

        let compare_register = Register {
            name: String::from(parts_iter.next().unwrap())
        };
        let compare_comparison = match parts_iter.next().unwrap() {
            ">" => Comparison::Gt,
            "<" => Comparison::Lt,
            ">=" => Comparison::Gte,
            "<=" => Comparison::Lte,
            "==" => Comparison::Eq,
            "!=" => Comparison::Ne,
            x => panic!(format!("Unknown comparison: {}", x)),
        };
        let compare_amount = parts_iter.next().unwrap().parse().unwrap();

        let input = Instruction {
            modify_register: modify_register,
            modify_direction: modify_direction, 
            modify_amount: modify_amount,
            compare_register: compare_register,
            compare_comparison: compare_comparison, 
            compare_amount: compare_amount,
        };

        input
    }).collect();

    println!("{:?}", input);

    let result = day8a(&input);
    println!("One: {}", result);
    /*
    let result = day6b(&input);
    println!("Two: {}", result);
    */
}

#[derive(Debug)]
enum Comparison {
    Lt,
    Gt,
    Lte,
    Gte,
    Eq,
    Ne,
}

impl Comparison {
    fn do_comparison(&self, reg: &i32, c: &i32) -> bool {
        match *self {
            Comparison::Lt => reg < c,
            Comparison::Gt => reg > c,
            Comparison::Lte => reg <= c,
            Comparison::Gte => reg >= c,
            Comparison::Eq => reg == c,
            Comparison::Ne => reg != c,
        }
    }
}

#[derive(Debug)]
enum ChangeDirection {
    Inc,
    Dec,
}

#[derive(Debug)]
struct Register {
    name: String
}

struct Instruction {
    modify_register: Register,
    modify_direction: ChangeDirection,
    modify_amount: i32,
    compare_register: Register,
    compare_comparison: Comparison,
    compare_amount: i32,
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?} {} if {} {:?} {}",
               &self.modify_register.name,
               &self.modify_direction,
               self.modify_amount,
               &self.compare_register.name,
               &self.compare_comparison,
               self.compare_amount)
    }
}

#[derive(Debug)]
struct Processor {
    registers: HashMap<String, i32>,
}

impl Processor {
    fn new() -> Processor {
        Processor { registers: HashMap::new() }
    }

    fn handle_instruction(&mut self, inst: &Instruction) -> () {
        /*
        let mut registers = &self.registers;
        let mod_reg_val = registers.entry(inst.modify_register.name.clone()).or_insert(0);
        let cmp_reg_val = registers.entry(inst.compare_register.name.clone()).or_insert(0);
        */
        if !self.registers.contains_key(&inst.modify_register.name) {
            self.registers.insert(inst.modify_register.name.clone(), 0);
        }
        if !self.registers.contains_key(&inst.compare_register.name) {
            self.registers.insert(inst.compare_register.name.clone(), 0);
        }

        let cmp_value = *self.registers.get(&inst.compare_register.name).unwrap();
        
        if inst.compare_comparison.do_comparison(&cmp_value, &inst.compare_amount) {
            let old_value = *self.registers.get(&inst.modify_register.name).unwrap();

            let new_value = match inst.modify_direction {
                ChangeDirection::Inc => old_value + inst.modify_amount,
                ChangeDirection::Dec => old_value - inst.modify_amount,
            };

            self.registers.insert(inst.modify_register.name.clone(), new_value);
        }
    }
}

fn day8a(input: &[Instruction]) -> i32 {
    let mut processor = Processor::new();

    for inst in input.iter() {
        processor.handle_instruction(inst);
    }

    println!("{:?}", processor);

    let registers = processor.registers;

    println!("{:?}", registers);

    let mut max = i32::min_value();
    for (_, val) in registers {
        if val > max {
            max = val;
        }
    }
    
    max
}
