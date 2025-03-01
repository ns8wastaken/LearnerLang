use std::str::Chars;
use crate::token::{TokenType, Token};


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

    pub fn advance(&mut self) {
        self.pos += 1;
        self.current_char = self.chars.next();
    }

    pub fn current_char(&self) -> char {
        match self.current_char {
            Some(c) => c,
            None => '\0',
        }
    }

    pub fn skip_whitespace(&mut self) {
        let mut c = self.current_char();

        while c == ' ' || c == '\t' || c == '\n' {
            self.advance();
            c = self.current_char();
        }
    }

    pub fn read_alpha_chars(&mut self) -> &str {
        let start = self.pos;

        let mut c = self.current_char();

        while c.is_alphabetic() || c == '_' {
            self.advance();
            c = self.current_char();
        }

        self.source.get(start..self.pos).unwrap()
    }

    pub fn read_digits(&mut self) -> &str {
        let start = self.pos;

        while self.current_char().is_ascii_digit() {
            self.advance();
        }

        self.source.get(start..self.pos).unwrap()
    }

    pub fn read_string(&mut self) -> &str {
        let start = self.pos;

        let mut c = self.current_char();

        while c != '"' && c != '\0' {
            self.advance();
            c = self.current_char();
        }

        self.advance();

        self.source.get(start..self.pos).unwrap()
    }

    pub fn read_comment(&mut self) -> &str {
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
                return Token::new(TokenType::Keyword, value.to_string());
            }

            return Token::new(TokenType::Identifier, value.to_string());
        }

        else if c.is_ascii_digit() {
            let value = self.read_digits();
            return Token::new(TokenType::Integer, value.to_string());
        }

        else {
            match c {
                '#' => {
                    let value = self.read_comment();
                    return Token::new(TokenType::Comment, value.to_string());
                }

                '=' => {
                    self.advance();
                    return Token::new(TokenType::Assignment, "=".to_string());
                }

                '+' => {
                    self.advance();
                    return Token::new(TokenType::Plus, "+".to_string());
                }

                '-' => {
                    self.advance();
                    return Token::new(TokenType::Minus, "-".to_string());
                }

                '*' => {
                    self.advance();
                    return Token::new(TokenType::Mult, "*".to_string());
                }

                '/' => {
                    self.advance();
                    return Token::new(TokenType::Div, "/".to_string());
                }

                '"' => {
                    let s = self.read_string();
                    return Token::new(TokenType::String, s.to_string());
                }

                '\0' => {
                    return Token::new(TokenType::EndOfFile, "✘".to_string());
                }

                _ => {}
            }
        }

        self.advance();
        Token::new(TokenType::Unknown, "✘".to_string())
    }
}
