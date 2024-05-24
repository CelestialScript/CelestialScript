#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    EOF,
    Let,
    Print,
    Identifier(String),
    Number(i64),
    Equals,
    Plus,
    Semicolon,
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            current_char: None,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        self.current_char = if self.read_position >= self.input.len() {
            None
        } else {
            Some(self.input.as_bytes()[self.read_position] as char)
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> i64 {
        let position = self.position;
        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[position..self.position].parse().unwrap()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.current_char {
            Some('=') => Token::Equals,
            Some('+') => Token::Plus,
            Some(';') => Token::Semicolon,
            Some(c) => {
                if c.is_alphabetic() {
                    let ident = self.read_identifier();
                    match ident.as_str() {
                        "let" => Token::Let,
                        "print" => Token::Print,
                        _ => Token::Identifier(ident),
                    }
                } else if c.is_digit(10) {
                    Token::Number(self.read_number())
                } else {
                    panic!("Unexpected character: {}", c)
                }
            }
            None => Token::EOF,
        };

        self.read_char();
        token
    }
}