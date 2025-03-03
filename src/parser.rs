use crate::token::Token;


#[derive(Debug)]
pub enum ASTNode {
    VarDeclaration(String, Box<ASTNode>),
    BinaryOp(char, Box<ASTNode>, Box<ASTNode>),
    Integer(i32),
    String(String),
    Identifier(String),
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

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous_token(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn consume(&mut self, token: Token, err_msg: &str) -> Result<&Token, String> {
        if self.current_token().unwrap().eq(&token) {
            self.advance();
            Ok(self.previous_token().unwrap())
        }
        else {
            Err(err_msg.to_string())
        }
    }

    // For parsing the primary tokens (int, string, identifiers...)
    fn parse_primary(&mut self) -> Result<ASTNode, String> {
        self.advance();
        
        match self.previous_token() {
            Some(Token::Integer(i)) => Ok(ASTNode::Integer(*i)),
            Some(Token::String(s)) => Ok(ASTNode::String(s.clone())),
            Some(Token::Identifier(name)) => Ok(ASTNode::Identifier(name.clone())),

            _ => Err("Expected an identifier.".to_string()),
        }
    }

    // Shunting yard
    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        let output_queue: Vec<Token> = Vec::new();
        let operator_stack: Vec<Token> = Vec::new();

        todo!()
    }

    // Parsing entry point
    pub fn parse(&mut self) -> Result<Vec<ASTNode>, String> {
        todo!()
    }
}
