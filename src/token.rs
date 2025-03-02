#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    KeywordVar,
    Identifier(&'a str),
    Integer(i32),
    String(&'a str),
    Comment(&'a str),

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


fn is_current_token(t1: &Token, t2: &Token) -> bool {
    std::mem::discriminant(t1) == std::mem::discriminant(t2)
}
