use std::io::{Read, self};
use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();

    // Get the input
    io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let buffer = buffer.trim();

    let lines : Vec<&str> = buffer.split('\n').collect();
    let input : Vec<Vec<&str>> = lines.iter().map(|row| {
        row.split_whitespace().collect()
    }).collect();


    let result = day4a(&input);
    println!("One: {}", result);
    let result = day4b(&input);
    println!("Two: {}", result);
}

fn valid_phrase(phrase : &[&str]) -> bool {
    let mut hashset = HashSet::new();

    for word in phrase {
        if hashset.contains(&word) {
            return false;
        }

        hashset.insert(word);
    }

    true
}

fn valid_anagram_phrase(phrase : &[&str]) -> bool {
    let mut hashset = HashSet::new();

    for mut word in phrase {
        let mut chars : Vec<char> = word.chars().collect();
        chars.sort();

        if hashset.contains(&chars) {
            return false;
        }

        hashset.insert(chars);
    }

    true
}

fn day4a(input : &[Vec<&str>]) -> usize {
    input.iter().map(|x| valid_phrase(x)).filter(|y| *y).count()
}

fn day4b(input : &[Vec<&str>]) -> usize {
    input.iter().map(|x| valid_anagram_phrase(x)).filter(|y| *y).count()
}
