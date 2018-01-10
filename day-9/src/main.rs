use std::io::Read;

mod lexer;

fn main() {
    let mut buffer = String::new();

    // Get the input
    std::io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let buffer = buffer.trim();

    let result = day9a(&buffer);
    println!("One: {}", result);
    let result = day9b(&buffer);
    println!("Two: {}", result);
}

fn day9a(input: &str) -> u32 {
    let lexer = lexer::Lexer::new(input.chars());

    let mut score = 0;
    let mut current_level = 0;

    for i in lexer {
        match i {
            lexer::Token::GroupStart => {
                current_level += 1;
                score += current_level;
            },
            lexer::Token::GroupEnd => {
                current_level -= 1;
            },
            _ => {},
        }
    }

    score
}

fn day9b(input: &str) -> usize {
    let lexer = lexer::Lexer::new(input.chars());
    let filtered = lexer.filter(|x| match *x {
        lexer::Token::Garbage(_) => true,
        _ => false,
    });

    filtered.count()
}
