mod generator;
mod remainder_iterator;

use remainder_iterator::IteratorRemainderExtension;

fn main() {
    // Test data
    // let a = 65;
    // let b = 8921;
    // Input data
    let a = 703;
    let b = 516;

    let result = day15a(a, b);
    println!("One: {}", result);
    let result = day15b(a, b);
    println!("Two: {}", result);
}

fn day15a(a: u64, b: u64) -> u64 {
    let a = generator::Generator::new_a(a).to_remainder(65536);
    let b = generator::Generator::new_b(b).to_remainder(65536);

    let combined = a.zip(b);

    let mut matches = 0;
    for (a, b) in combined.take(40000000) {
        if a == b {
            matches += 1;
        }
    }

    matches
}

fn day15b(a: u64, b: u64) -> u64 {
    let a = generator::Generator::new_a(a).filter(|x| x % 4 == 0).to_remainder(65536);
    let b = generator::Generator::new_b(b).filter(|x| x % 8 == 0).to_remainder(65536);

    let combined = a.zip(b);

    let mut matches = 0;
    for (a, b) in combined.take(5000000) {
        if a == b {
            matches += 1;
        }
    }

    matches
}
