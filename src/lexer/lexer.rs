use std::collections::HashMap;

use crate::{
    error::LexerError,
    keywords,
    token::{ Token, TokenType }
};

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    curr: usize,
    line: usize,
    keywords: HashMap<String, TokenType>
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut kw: HashMap<String, TokenType> = HashMap::new();
        keywords!(
            kw;
            And, Class, Else, False, For, Fun, If, Nil, Or,
            Print, Return, Super, This, True, Var, While
        );
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            curr: 0,
            line: 1,
            keywords: kw,
        }
    }

    pub fn run(&mut self) -> Result<Vec<Token>, LexerError> {
        while !self.is_at_end() {
            self.start = self.curr;
            if let Err(e) = self.scan_token() {
                return Err(e);
            }
        }
        self.tokens.push(
            Token::new(
                TokenType::Eof,
                "".to_string(),
                "".to_string(),
                self.line,
                self.start,
                self.curr
            )
        );
        Ok(self.tokens.clone())
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = String::from(&self.source[self.start..self.curr]);
        self.tokens.push(Token::new(token_type, text, String::new(), self.line, self.start, self.curr));
    }

    fn add_string_token(&mut self, token_type: TokenType, literal: String) {
        let text = String::from(&self.source[self.start..self.curr]);
        self.tokens.push(Token::new(token_type, text, literal, self.line, self.start, self.curr));
    }

    fn string(&mut self) -> Result<(), LexerError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            if let Err(e) = self.advance() {
                return Err(e);
            }
        }

        if self.is_at_end() {
            return Err(LexerError::UnterminatedString{ line: self.line });
        }

        if let Err(e) = self.advance() {
            return Err(e);
        }

        let value: String = String::from(&self.source[self.start + 1..self.curr - 1]);
        self.add_string_token(TokenType::String, value);
        Ok(())
    }

    fn number(&mut self) -> Result<(), LexerError> {
        while self.is_digit(self.peek()) {
            if let Err(e) = self.advance() {
                return Err(e)
            }
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            if let Err(e) = self.advance() {
                return Err(e);
            }

            while self.is_digit(self.peek()) {
                if let Err(e) = self.advance() {
                    return Err(e);
                }
            }
        }
        
        let value = String::from(&self.source[self.start..self.curr]);
        self.add_string_token(TokenType::Num, value);
        Ok(())
    }

    fn identifier(&mut self) -> Result<(), LexerError> {
        while self.is_alpha(self.peek()) {
            if let Err(e) = self.advance() {
                return Err(e);
            }
        }

        let text = String::from(&self.source[self.start..self.curr]);
        let token_type: TokenType = match self.keywords.get(&text) {
            Some(v) => v.clone(),
            None => TokenType::Identifier,
        };

        self.add_token(token_type);
        Ok(())
    }

    fn comment(&mut self) -> Result<(), LexerError> {
        if self.match_token('/') {
            loop {
                if self.peek() != '\n' && !self.is_at_end() {
                    if let Err(e) = self.advance() {
                        return Err(e);
                    }
                } else {
                    self.tokens.push(
                        Token::new(
                            TokenType::Comment,
                            String::from(self.source[self.start + 2..self.curr].trim()),
                            String::from(self.source[self.start..self.curr].trim()),
                            self.line,
                            self.start,
                            self.curr
                        )
                    );
                    return Ok(());
                }
            }
        } else {
            self.add_token(TokenType::FSlash);
            Ok(())
        }
    }

    fn scan_token(&mut self) -> Result<(), LexerError> {
        let c_result = self.advance();
        let token: TokenType;
        let c = if let Ok(cha) = c_result {
            cha
        } else {
            '\0'
        };
        if let Err(e) = c_result {
            return Err(e);
        };
        match c {
            '(' => token = TokenType::LParen,
            ')' => token = TokenType::RParen,
            '{' => token = TokenType::LBrace,
            '}' => token = TokenType::RBrace,
            ',' => token = TokenType::Comma,
            '.' => token = TokenType::Dot,
            ';' => token = TokenType::Semicolon,
            '*' => token = TokenType::Asterisk,
            '-' => {
                if self.match_token('-') {
                    token = TokenType::Decr;
                } else {
                    token = TokenType::Minus;
                }
            },
            '+' => {
                if self.match_token('+') {
                    token = TokenType::Incr;
                } else {
                    token = TokenType::Plus;
                }
            },
            '!' => {
                if self.match_token('=') {
                    token = TokenType::BangEqual;
                } else {
                    token = TokenType::Bang;
                }
            },
            '=' => {
                if self.match_token('=') {
                    token = TokenType::EqualEqual;
                } else {
                    token = TokenType::Equal;
                }
            },
            '<' => {
                if self.match_token('=') {
                    token = TokenType::LessEqual;
                } else {
                    token = TokenType::Less;
                }
            },
            '>' => {
                if self.match_token('=') {
                    token = TokenType::GreaterEqual;
                } else {
                    token = TokenType::Greater;
                }
            },
            '\n' => {
                self.line += 1;
                return Ok(());
            },
            ' ' | '\r' | '\t' => return Ok(()),
            '/' => {
                if let Err(e) = self.comment() {
                    return Err(e);
                }
                return Ok(());
            },
            '"' => {
                return match self.string() {
                    Err(e) => Err(e),
                    Ok(()) => Ok(())
                };
            },
            _ => {
                if self.is_digit(c) {
                    if let Err(e) = self.number() {
                        return Err(e);
                    }
                } else if self.is_alpha(c) {
                    if let Err(e) = self.identifier() {
                        return Err(e);
                    }
                } else {
                    return Err(LexerError::UnexpectedCharacter{ c, line: self.line });
                }
                return Ok(());
            }
        }
        self.add_token(token);
        Ok(())
    }

    fn advance(&mut self) -> Result<char, LexerError> {
        return if let Some(c) = self.source.chars().nth(self.curr) {
            self.curr += 1;
            Ok(c)
        } else {
            Err(LexerError::NoCharactersLeft{ line: self.line })
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.curr).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.curr + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.curr + 1).unwrap()
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false };

        if self.source.chars().nth(self.curr).unwrap() != expected {
            return false;
        };

        self.curr += 1;
        true
    }

    fn is_digit(&mut self, c: char) -> bool {
        c.is_ascii_digit()
    }
    
    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() ||
        (c == '_')
    }

    fn is_at_end(&self) -> bool {
        self.curr >= self.source.len()
    }
}