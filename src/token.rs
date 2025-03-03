#[derive(Debug)]
pub enum Token {
    KeywordVar,
    Semicolon,
    Identifier(String),
    Integer(i32),
    String(String),
    Comment(String),

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


impl Token {
    pub fn eq(&self, t: &Token) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(t)
    }
}
