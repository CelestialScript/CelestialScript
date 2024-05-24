use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Identifier(String),
    BinaryOp(Box<Expr>, BinaryOperator, Box<Expr>),
}

#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
}

#[derive(Debug)]
pub enum Statement {
    Let(String, Expr),
    Print(Expr),
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while self.current_token != Token::EOF {
            let stmt = self.parse_statement();
            statements.push(stmt);
            self.next_token();
        }
        statements
    }

    fn parse_statement(&mut self) -> Statement {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Print => self.parse_print_statement(),
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn parse_let_statement(&mut self) -> Statement {
        self.next_token(); // Consume 'let'
        if let Token::Identifier(ref name) = self.current_token {
            self.next_token(); // Consume identifier
            self.expect_token(Token::Equals);
            self.next_token(); // Consume '='
            let expr = self.parse_expression();
            self.expect_token(Token::Semicolon);
            Statement::Let(name.clone(), expr)
        } else {
            panic!("Expected identifier, found: {:?}", self.current_token);
        }
    }

    fn parse_print_statement(&mut self) -> Statement {
        self.next_token(); // Consume 'print'
        let expr = self.parse_expression();
        self.expect_token(Token::Semicolon);
        Statement::Print(expr)
    }

    fn parse_expression(&mut self) -> Expr {
        let left = self.parse_primary();
        self.parse_binary_op_rhs(0, left)
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current_token {
            Token::Number(n) => {
                self.next_token(); // Consume number
                Expr::Number(n)
            }
            Token::Identifier(ref name) => {
                self.next_token(); // Consume identifier
                Expr::Identifier(name.clone())
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn parse_binary_op_rhs(&mut self, precedence: i32, mut left: Expr) -> Expr {
        while let Some(op) = self.current_binary_operator() {
            let op_precedence = self.get_precedence(op);
            if op_precedence < precedence {
                return left;
            }
            self.next_token(); // Consume operator
            let mut right = self.parse_primary();
            while let Some(next_op) = self.current_binary_operator() {
                let next_precedence = self.get_precedence(next_op);
                if op_precedence < next_precedence {
                    right = self.parse_binary_op_rhs(op_precedence + 1, right);
                } else {
                    break;
                }
            }
            left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
        }
        left
    }

    fn current_binary_operator(&self) -> Option<BinaryOperator> {
        match self.current_token {
            Token::Plus => Some(BinaryOperator::Plus),
            _ => None,
        }
    }

    fn get_precedence(&self, op: BinaryOperator) -> i32 {
        match op {
            BinaryOperator::Plus => 10,
        }
    }

    fn expect_token(&mut self, token: Token) {
        if self.current_token == token {
            self.next_token();
        } else {
            panic!("Expected token: {:?}, found: {:?}", token, self.current_token);
        }
    }
}