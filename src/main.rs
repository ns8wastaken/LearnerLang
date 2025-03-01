mod token;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;


fn main() {
    let source_code =
r#"var i = 20"#;

    let mut lexer = Lexer::new(source_code);

    println!("{}", lexer.read_next_token());
    println!("{}", lexer.read_next_token());
    println!("{}", lexer.read_next_token());
    println!("{}", lexer.read_next_token());
}
