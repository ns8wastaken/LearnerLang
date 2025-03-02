use crate::token::Token;


pub enum ASTNode {
    VarDeclaration(String, Box<ASTNode>),
    BinaryOp(char, Box<ASTNode>, Box<ASTNode>),
    Number(i32),
    Variable(String),
}


pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}


impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn consume(&mut self) {
        
    }
}
