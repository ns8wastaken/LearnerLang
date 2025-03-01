mod token;
mod lexer;

use token::Token;
use lexer::Lexer;


fn main() {
    let source_code =
r#"var i = 20"#;

    let mut lexer = Lexer::new(source_code);

    println!("{}", lexer.read_next_token().value);
    println!("{}", lexer.read_next_token().value);
    println!("{}", lexer.read_next_token().value);
    println!("{}", lexer.read_next_token().value);
}
