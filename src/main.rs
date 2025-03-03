mod token;
mod lexer;
mod parser;

use token::Token;
use lexer::Lexer;
use parser::Parser;


fn main() {
    let source_code =
r#"var i = 20;
var j = 20 - 10
var k = i + j;"#;

    let mut lexer = Lexer::new(source_code);
    let tokens = lexer.get_tokens();

    for t in &tokens {
        println!("{:?}", t);
    }

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => println!("Parsed AST: {:?}", ast),
        Err(e) => println!("Error: {}", e),
    }
}
