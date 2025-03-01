use crate::token::Token;


pub enum ASTNode {
    VarDeclaration(String, Box<ASTNode>),
    BinaryOp(char, Box<ASTNode>, Box<ASTNode>),
    Number(i32),
    Variable(String),
}


pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(self.current).expect("How?")
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn current_token_is(&self, token: &Token) -> bool {
        std::mem::discriminant(self.current_token()) == std::mem::discriminant(token)
    }

    fn consume(&mut self) {
        todo!()
    }
}
