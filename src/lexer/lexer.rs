use std::collections::HashMap;

use crate::{
    keywords,
    interpreter,
    tokens::{Token, TokenType}
};

pub struct Lexer {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    curr: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        // Initialise a hashmap containing all the keywords of the language
        let mut kw: HashMap<String, TokenType> = HashMap::new();

        keywords!(
            kw ;
            And, Class, Else, False, For, Fun, If, Nil, Or,
            Print, Return,Super, This, True, Var, While
        );

        Self {
            source,
            tokens: Vec::new(), // Initialise an empty vector for the tokens
            // Initialise the pointers
            start: 0,
            curr: 0,
            line: 1,
            keywords: kw,
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

    fn is_at_end(&self) -> bool {
        self.curr >= self.source.len()
    }

    // Advance to the next token
    fn advance(&mut self) -> char {
        return if let Some(c) = self.source.chars().nth(self.curr) {
            self.curr += 1;
            c
        } else {
            self.error(self.line, "No more characters left?");
            '\0'
        }
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        };

        if self.source.chars().nth(self.curr).unwrap() != expected {
            return false;
        }

        self.curr += 1;
        true
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = String::from(&self.source[self.start..self.curr]);
        self.tokens.push(Token::new(token_type, text, String::new(), self.line, self.start, self.curr));
    }

    fn add_string_token(&mut self, token_type: TokenType, literal: String) {
        let text = String::from(&self.source[self.start..self.curr]);
        self.tokens.push(Token::new(token_type, text, literal, self.line, self.start, self.curr));
    }
    
    fn is_digit(&mut self, c: char) -> bool {
        c.is_ascii_digit()
    }
    
    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() ||
        (c == '_')
    }
    
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error(self.line, "Unterminated string.");
        }

        self.advance();

        let value: String = String::from(&self.source[self.start + 1..self.curr - 1]);
        self.add_string_token(TokenType::String, value);
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        
        let value = String::from(&self.source[self.start..self.curr]);
        self.add_string_token(TokenType::Num, value);
    }

    fn identifier(&mut self) {

        while self.is_alpha(self.peek()) {
            self.advance();
        }

        let text = String::from(&self.source[self.start..self.curr]);
        let token_type: TokenType = match self.keywords.get(&text) {
            Some(v) => *v,
            None => TokenType::Identifier,
        };

        self.add_token(token_type);
    }

    fn comment(&mut self) {
        if self.match_token('/') {
            loop {
                if self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
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
                    return;
                }
            }
        } else {
            self.add_token(TokenType::FSlash);
        }
    }

    // Scans the next part of the source code and generates a token from it
    fn scan_token(&mut self) {
        let c: char = self.advance(); // Gets the next character from the input
        let token: TokenType; // Initialise the token
        match c {
            '(' => token = TokenType::LeftParen,
            ')' => token = TokenType::RightParen,
            '{' => token = TokenType::LeftBrace,
            '}' => token = TokenType::RightBrace,
            ',' => token = TokenType::Comma,
            '.' => token = TokenType::Dot,
            '-' => token = TokenType::Minus,
            '+' => token = TokenType::Plus,
            ';' => token = TokenType::Semicolon,
            '*' => token = TokenType::Asterisk,
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
            ' ' | '\r' | '\t' | '\n' => {
                return;
            },
            '/' => {
                self.comment();
                return;
            },
            '"' => {
                self.string();
                return;
            },
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    interpreter::Interpreter::line_error(self.line, "Unexpected character.");
                }
                return;
            }
        }
        self.add_token(token);
    }

    fn _print_tokens(&self) {
        for token in &self.tokens {
            println!("{token}");
        }
    }

    // Scans the raw source code and generates a vector of tokens from it
    pub fn scan(&mut self) -> Vec<Token> {
        while !self.is_at_end() { // Runs until the lexer reaches the end of the inputted code
            // Set start ptr to the current ptr's value, because the current ptr is at the start of the next token
            self.start = self.curr;
            self.scan_token(); // Scans the next token
        };
        
        // Once it has reached the end, add the EoF token to the tokens vector
        self.tokens.push(
            Token::new(TokenType::Eof, String::new(), String::new(), self.line, self.start, self.curr)
        );
        self.tokens.clone() // Return the tokens vector with all the generated token
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }
    
    fn report(&mut self, line: usize, where_about: &str, message: &str) {
        println!("[line {line}] Error {where_about}: {message}");
    }
}