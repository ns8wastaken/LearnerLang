use std::str::Chars;
use crate::token::Token;


fn is_keyword(value: &str) -> bool {
    match value {
        "var" => true,
        _ => false,
    }
}


pub struct Lexer<'a> {
    source: &'a str,
    pos: usize,
    chars: Chars<'a>,
    current_char: Option<char>,
}


impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        let mut chars = source.chars();
        let chr = chars.next();
        Self {
            source,
            pos: 0,
            chars,
            current_char: chr,
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.current_char = self.chars.next();
    }

    fn current_char(&self) -> char {
        match self.current_char {
            Some(c) => c,
            None => '\0',
        }
    }

    fn skip_whitespace(&mut self) {
        let mut c = self.current_char();

        while c == ' ' || c == '\t' || c == '\n' {
            self.advance();
            c = self.current_char();
        }
    }

    fn read_alpha_chars(&mut self) -> &str {
        let start = self.pos;

        let mut c = self.current_char();

        while c.is_alphabetic() || c == '_' {
            self.advance();
            c = self.current_char();
        }

        self.source.get(start..self.pos).unwrap()
    }

    fn read_digits(&mut self) -> &str {
        let start = self.pos;

        while self.current_char().is_ascii_digit() {
            self.advance();
        }

        self.source.get(start..self.pos).unwrap()
    }

    fn read_string(&mut self) -> &str {
        let start = self.pos;

        let mut c = self.current_char();

        while c != '"' && c != '\0' {
            self.advance();
            c = self.current_char();
        }

        self.advance();

        self.source.get(start..self.pos).unwrap()
    }

    fn read_comment(&mut self) -> &str {
        let start = self.pos;

        while self.current_char() != '\n' {
            self.advance();
        }

        self.advance();

        self.source.get(start..self.pos - 1).unwrap()
    }

    pub fn read_next_token(&mut self) -> Token {
        self.skip_whitespace();

        let c = self.current_char();

        if c.is_alphabetic() || c == '_' {
            let value = self.read_alpha_chars();

            if is_keyword(value) {
                return Token::KeywordVar;
            }

            return Token::Identifier(value);
        }

        else if c.is_ascii_digit() {
            let value = self.read_digits();
            return Token::Integer(value.parse().expect("Somehow an int isn't an int"));
        }

        else {
            match c {
                '#' => {
                    let value = self.read_comment();
                    return Token::Comment(value);
                }

                '=' => {
                    self.advance();
                    return Token::Assignment;
                }

                '+' => {
                    self.advance();
                    return Token::Plus;
                }

                '-' => {
                    self.advance();
                    return Token::Minus;
                }

                '*' => {
                    self.advance();
                    return Token::Mult;
                }

                '/' => {
                    self.advance();
                    return Token::Div;
                }

                '"' => {
                    let s = self.read_string();
                    return Token::String(s);
                }

                '\0' => {
                    return Token::EndOfFile;
                }

                _ => {}
            }
        }

        self.advance();
        Token::Unknown
    }

    //pub fn get_tokens(&mut self) -> Vec<Token> {
    //    let mut tokens: Vec<Token> = Vec::new();
    //
    //    loop {
    //        let token = self.read_next_token();
    //        tokens.push(token);
    //
    //        if tokens.last().unwrap() == &Token::EndOfFile {
    //            break;
    //        }
    //    }
    //
    //    tokens
    //}
}
