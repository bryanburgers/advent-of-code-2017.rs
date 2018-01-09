use std::str::Chars;

#[derive(Debug)]
pub enum Token {
    GroupStart,
    GroupEnd,
    Separator,
    Char(char),
    GarbageStart,
    Garbage(char),
    CancelledGarbage(char),
    GarbageEnd,
}

enum LexerState {
    OutOfGarbage,
    InGarbage,
}

pub struct Lexer<'a> {
    state: LexerState,
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: Chars<'a>) -> Lexer<'a> {
        Lexer { state: LexerState::OutOfGarbage, chars: chars }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let c = match self.chars.next() {
            Some(c) => c,
            None => return None,
        };

        match self.state {
            LexerState::OutOfGarbage => {
                match c {
                    '{' => Some(Token::GroupStart),
                    '}' => Some(Token::GroupEnd),
                    ',' => Some(Token::Separator),
                    '>' => panic!("Unexpected character END_OF_GARBAGE"),
                    '!' => panic!("Unexpected character EXCLAMATION_POINT"),
                    '<' => {
                        self.state = LexerState::InGarbage;
                        Some(Token::GarbageStart)
                    }
                    c => Some(Token::Char(c))
                }
            },
            LexerState::InGarbage => {
                match c {
                    '>' => {
                        self.state = LexerState::OutOfGarbage;
                        Some(Token::GarbageEnd)
                    },
                    '!' => {
                        // Escape character. We need to get the next character.
                        let next_char = self.chars.next();
                        match next_char {
                            None => panic!("End of input after an escape!"),
                            Some(c) => Some(Token::CancelledGarbage(c)),
                        }
                    },
                    c => Some(Token::Garbage(c)),
                }
            },
        }
    }
}
