#[derive(Debug, PartialEq)]
pub enum Token {
    Let,
    Print,
    Identifier(String),
    Number(i64),
    Plus,
    Equals,
    Semicolon,
    EOF,
}

pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            current_char: None,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.input.chars().nth(self.position).unwrap());
        }
        self.position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.current_char {
            Some('=') => Token::Equals,
            Some('+') => Token::Plus,
            Some(';') => Token::Semicolon,
            Some(c) if c.is_digit(10) => self.read_number(),
            Some(c) if c.is_alphabetic() => self.read_identifier(),
            Some(_) => {
                self.read_char();
                return self.next_token();
            }
            None => Token::EOF,
        };
        self.read_char();
        token
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

    fn read_number(&mut self) -> Token {
        let start_position = self.position - 1;
        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                self.read_char();
            } else {
                break;
            }
        }
        let number_str: String = self.input[start_position..self.position - 1].to_string();
        Token::Number(number_str.parse::<i64>().unwrap())
    }

    fn read_identifier(&mut self) -> Token {
        let start_position = self.position - 1;
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() {
                self.read_char();
            } else {
                break;
            }
        }
        let identifier_str: String = self.input[start_position..self.position - 1].to_string();
        match identifier_str.as_str() {
            "let" => Token::Let,
            "print" => Token::Print,
            _ => Token::Identifier(identifier_str),
        }
    }
}