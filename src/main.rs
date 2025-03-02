mod token;
mod lexer;
mod parser;

use token::Token;
use lexer::Lexer;
use parser::Parser;


fn main() {
    let source_code =
r#"var i = 20"#;

    let mut lexer = Lexer::new(source_code);
    let mut token_vec: Vec<Token> = Vec::new();

    loop {
        let token = lexer.read_next_token();
        token_vec.push(token);

        if let Some(last_token) = token_vec.last() {
            if last_token == &Token::EndOfFile {
                break;
            }
        }
    }

    let mut parser = Parser::new(token_vec);

    //println!("{:?}", lexer.read_next_token());
    //println!("{:?}", lexer.read_next_token());
    //println!("{:?}", lexer.read_next_token());
    //println!("{:?}", lexer.read_next_token());
}
