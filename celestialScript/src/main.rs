mod lexer;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let input = String::from("
        let x = 10;
        let y = 20;
        print x + y;
    ");

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let statements = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.interpret(statements);
}