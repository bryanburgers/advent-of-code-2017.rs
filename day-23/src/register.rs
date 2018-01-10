use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl Register {
    fn parse_maybe(str: &str) -> Option<Register> {
        match str {
            "a" => Some(Register::A),
            "b" => Some(Register::B),
            "c" => Some(Register::C),
            "d" => Some(Register::D),
            "e" => Some(Register::E),
            "f" => Some(Register::F),
            "g" => Some(Register::G),
            "h" => Some(Register::H),
            _ => None,
        }
    }

    fn parse(str: &str) -> Register {
        match Register::parse_maybe(str) {
            Some(r) => r,
            None => panic!("Unexpected register {}", str),
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Constant(i64),
    Reference(Register),
}

impl Value {
    fn parse(str: &str) -> Value {
        match Register::parse_maybe(str) {
            Some(r) => Value::Reference(r),
            None => Value::Constant(str.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Set(Register, Value),
    Sub(Register, Value),
    Mul(Register, Value),
    Jnz(Value, Value),
}

impl Instruction {
    pub fn parse(str: &str) -> Instruction {
        let parts : Vec<&str> = str.split_whitespace().collect();

        match parts[0] {
            "set" => Instruction::Set(Register::parse(parts[1]), Value::parse(parts[2])),
            "sub" => Instruction::Sub(Register::parse(parts[1]), Value::parse(parts[2])),
            "mul" => Instruction::Mul(Register::parse(parts[1]), Value::parse(parts[2])),
            "jnz" => Instruction::Jnz(Value::parse(parts[1]), Value::parse(parts[2])),
            v => panic!("Unexpected instruction {}", v),
        }
    }
}

#[derive(Debug)]
pub struct Coprocessor<'a> {
    registers: HashMap<Register, i64>,
    instructions: &'a[Instruction],
    instruction_pointer: i64,
    multiplications: usize,
}

impl<'a> Coprocessor<'a> {
    pub fn new(instructions: &'a[Instruction]) -> Coprocessor {
        let mut registers = HashMap::new();
        registers.insert(Register::A, 0);
        registers.insert(Register::B, 0);
        registers.insert(Register::C, 0);
        registers.insert(Register::D, 0);
        registers.insert(Register::E, 0);
        registers.insert(Register::F, 0);
        registers.insert(Register::G, 0);
        registers.insert(Register::H, 0);

        Coprocessor {
            registers: registers,
            multiplications: 0,
            instructions: instructions,
            instruction_pointer: 0
        }
    }

    pub fn is_done(&self) -> bool {
        self.instruction_pointer < 0 || self.instruction_pointer >= (self.instructions.iter().count() as i64)
    }

    pub fn multiplications(&self) -> usize {
        self.multiplications
    }

    pub fn step(&mut self) -> Option<()> {
        if self.is_done() {
            return None;
        }

        let instruction = &self.instructions[self.instruction_pointer as usize];
        // print!("{} ", self.instruction_pointer);

        self.perform_instruction(instruction);

        Some(())
    }

    fn reg_value(&self, reg: &Register) -> i64 {
        *self.registers.get(&reg).unwrap()
    }

    fn value(&self, val: &Value) -> i64 {
        match *val {
            Value::Constant(c) => c,
            Value::Reference(ref r) => {
                *self.registers.get(&r).unwrap()
            },
        }
    }

    fn perform_instruction(&mut self, inst: &Instruction) {
        match inst {
            &Instruction::Set(ref reg, ref val) => {
                let val = self.value(&val);
                // println!("[set] setting {:?} to {}", reg, val);
                self.registers.insert(reg.clone(), val);
                self.instruction_pointer += 1;
            },
            &Instruction::Sub(ref reg, ref val) => {
                let cur = self.reg_value(&reg);
                let val = self.value(&val);
                // println!("[sub] setting {:?} to {}", reg, cur - val);
                self.registers.insert(reg.clone(), cur - val);
                self.instruction_pointer += 1;
            },
            &Instruction::Mul(ref reg, ref val) => {
                let cur = self.reg_value(&reg);
                let val = self.value(&val);
                // println!("[mul] setting {:?} to {}", reg, cur * val);
                self.registers.insert(reg.clone(), cur * val);
                self.multiplications += 1;
                self.instruction_pointer += 1;
            },
            &Instruction::Jnz(ref val1, ref val2) => {
                let cur = self.value(&val1);
                // println!("[jnz]");
                match cur {
                    0 => {
                        self.instruction_pointer += 1;
                    },
                    _ => {
                        self.instruction_pointer += self.value(&val2);
                    },
                };
            },
        }
    }
}
