pub enum TokenType {
    Keyword,
    Identifier,
    Integer,
    String,
    Comment,

    // Operators
    Assignment,
    Plus,
    Minus,
    Mult,
    Div,
    Pow,
    Not,
    BinaryOr,
    BinaryAnd,
    BinaryXor,
    BinaryNot,

    EndOfFile,
    Unknown
}


pub struct Token {
    t_type: TokenType,
    pub value: String,
    start: usize,
    end: usize,
}


impl Token {
    pub fn new(t_type: TokenType, value: String) -> Token {
        Self {
            t_type,
            value,
            start: 0,
            end: 0,
        }
    }
}
