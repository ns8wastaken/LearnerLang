pub enum Token {
    KeywordVar,
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
