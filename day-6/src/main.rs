use std::io::{Read, self};
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();

    // Get the input
    io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let buffer = buffer.trim();

    let input : Vec<u16> = buffer.split_whitespace().map(|line| line.parse().unwrap()).collect();

    // println!("{:?}", input);


    let result = day6a(&input);
    println!("One: {}", result);
    let result = day6b(&input);
    println!("Two: {}", result);
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct MemoryBank {
    blocks: u16
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Memory {
    banks: Vec<MemoryBank>,
}

impl Memory {
    fn new(blocks: &[u16]) -> Memory {
        let mut banks = Vec::new();

        for block in blocks {
            banks.push(MemoryBank { blocks: *block })
        }

        Memory { banks: banks }
    }

    fn largest_index(&self) -> usize {
        let mut i = 0;
        let mut max = 0;
        for (index, bank) in self.banks.iter().enumerate() {
            if bank.blocks > max {
                i = index;
                max = bank.blocks;
            }
        }

        i
    }

    fn redistribute(&mut self) {
        let largest_index = self.largest_index();

        let mut blocks = self.banks[largest_index].blocks;

        self.banks[largest_index].blocks = 0;

        let mut i = largest_index;
        while blocks > 0 {
            i = (i + 1) % self.banks.len();

            self.banks[i].blocks += 1;

            blocks -= 1;
        }
    }
}

fn day6a(input: &[u16]) -> usize {
    let mut mem = Memory::new(input);
    let mut hs = HashSet::new();
    let mut count = 0;

    let l = format!("{:?}", &mem);
    hs.insert(l);

    loop {
        count += 1;
        mem.redistribute();

        let l = format!("{:?}", &mem);
        if hs.contains(&l) {
            return count;
        }

        hs.insert(l);
    }
}

fn day6b(input: &[u16]) -> usize {
    let mut mem = Memory::new(input);
    let mut hm = HashMap::new();
    let mut count = 0;

    let l = format!("{:?}", &mem);
    hm.insert(l, 0);

    loop {
        count += 1;
        mem.redistribute();

        let l = format!("{:?}", &mem);
        if hm.contains_key(&l) {
            return count - *hm.get(&l).unwrap();
        }

        hm.insert(l, count);
    }
}
