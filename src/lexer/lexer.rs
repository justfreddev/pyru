//! The lexer module is the first processing step of the interpreter and is responsible for
//! converting the source code into tokens through a process called tokenization. 
//! 
//! The `Lexer` struct represents the lexer and is responsible for converting the source code
//! into tokens. It takes in the source code as a string and outputs a vector of tokens that
//! represent the source code.
//! 
//! ## Example
//! 
//! ```rust
//! use crate::lexer::Lexer;
//! use crate::token::TokenType;
//! 
//! let source_code = r#"
//!     var x = 10;
//!     if (x > 5) {
//!         print "Hello, world!";
//!     }
//! "#;
//! 
//! let mut lexer = Lexer::new(source_code.to_string());
//! let tokens = lexer.run().unwrap();
//! 
//! for token in tokens {
//!     println!("{:?}", token);
//! }
//! ```
//! 
//! ## The Process
//! 
//! Lexical analysis works as follows:
//! 
//! 1. The lexer reads the source code character by character
//! 2. It matches the character to a specific token such as identifiers, operators, numbers, etc.
//! 3. It creates a Token object for each token in the source, which contains information about its
//! position, contents, and type.
//! 4. The lexer continues to process the source code and tokenizes it until it reaches the end of
//! the source.
//! 5. Finally, it returns the vector of tokens that represent the source code.
//! 
//! However, the source code is not just made up of characters that each individually represent
//! tokens. It also contains:
//! - Operators: `==` and `++`
//! - Comments: `// This is a comment`
//! - Identifiers: `foo` and `bar`
//! - Literals: `"Hello World!"` and `123.456`
//! - Keywords: `if`, `else` and `def`
//! 
//! To process these, it uses the ability to peek (check what the next character is) and advance
//! (move on to the next character) to match the token to one of these more complex tokens.
//! 
//! ## Errors
//! 
//! If the lexer detects something wrong within the source code, such as an unexpected character or
//! an unterminated literal, then it will return `Err(LexerError)`.


use std::collections::HashMap;

use crate::{
    error::LexerError,
    keywords,
    token::{Token, TokenType},
};


/// Carries out the lexical analysis process.
/// 
/// ## Fields
/// 
/// - `source`: The source code as a [`String`]
/// - `tokens`: A vector of tokens that represent the source code
/// - `start`: The starting index of the current token being processed
/// - `curr`: The current index of the lexer's position in the source code
/// - `line`: The current line number in the source code
/// - `keywords`: A HashMap that maps keyword strings to their corresponding [`TokenType`]
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    curr: usize,
    line: usize,
    indent: usize,
    is_indented: bool,
    is_new_line: bool,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    /// Returns a new instance of the Lexer struct
    /// ## Arguments
    /// - `source`: The source code being interpreted
    /// ## Returns
    /// - `Lexer`: A new instance of the lexer
    pub fn new(source: String) -> Self {
        // Creates a new HashMap, mapping keyword Strings to the
        // TokenType of the keyword of all the keywords of the language
        let mut kw: HashMap<String, TokenType> = HashMap::new();
        keywords!(
            kw;
            And, Def, Else, False, For, If, In, Null,
            Or, Print, Return, Step, True, Var, While
        );

        return Self {
            source,
            tokens: Vec::new(),
            start: 0,
            curr: 0,
            line: 1,
            indent: 0,
            is_indented: false,
            is_new_line: false,
            keywords: kw,
        };
    }

    /// Runs the lexer and tokenizes `self.source`.
    /// 
    /// It works by setting the `start` pointer scanning tokens with `scan_token()` until the end
    /// of the source code, when it will add the EoF (End of File) token and return
    /// `Ok(Vec<Token>)`, or `Err(LexerError)`
    /// 
    /// ## Returns
    /// - [`Result<Vec<Token>, LexerError>`]: Either successfully returns the vector of tokens, or
    /// a `LexerError` where something has led to an error during the scanning process.
    pub fn run(&mut self) -> Result<Vec<Token>, LexerError> {
        while !self.is_at_end() {

            // Resets the start pointer to the current position to be ready for a new token
            self.start = self.curr;

            // Scans the source for for the next token, and returns an error if one occurred
            self.scan_token()?;
        }
        self.start = self.curr;

        if self.is_indented {
            self.tokens.push(Token::new(
                TokenType::Dedent,
                "".to_string(),
                "".to_string(),
                self.line,
                self.start,
                self.curr
            ));
        }

        // Adds the End of File token to mark the end of the source code
        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            "".to_string(),
            self.line,
            self.start,
            self.curr,
        ));

        return Ok(self.tokens.clone());
    }

    /// Adds a token to `self.tokens`
    /// 
    /// ## Arguments
    /// - `token_type`: The type of the token to be added, determined from `scan_token()`
    fn add_token(&mut self, token_type: TokenType) {
        let text = String::from(&self.source[self.start..self.curr]);
        self.tokens.push(Token::new(
            token_type,
            text,
            String::new(),
            self.line,
            self.start,
            self.curr,
        ));
    }

    /// Adds a string or number token to `self.tokens`.
    /// 
    /// ## Arguments
    /// - `token_type`: The type of token being added
    /// - `literal`: The literal value of the token being added, such as `1234` or `Hello World`
    fn add_string_token(&mut self, token_type: TokenType, literal: String) {
        let text = String::from(&self.source[self.start..self.curr]);
        self.tokens.push(Token::new(
            token_type, text, literal, self.line, self.start, self.curr,
        ));
    }

    /// Processes a string token once `"` is found, and repeatedly advances, as long as another `"`
    /// is found or the end of the source code is not reached.
    /// 
    /// ## Returns
    /// [`Result<(), LexerError>`]: Either successfully returns nothing once the string token is
    /// processed and pushed to the tokens vector or returns a [`LexerError`] if an error is
    /// encountered
    fn string(&mut self) -> Result<(), LexerError> {
        while self.peek()? != '"' && !self.is_at_end() {
            if self.peek()? == '\n' {
                return Err(LexerError::UnterminatedString {
                    line: self.line,
                    start: self.start,
                    end: self.curr
                });
            }
            self.advance()?;
        }

        if self.is_at_end() {
            return Err(LexerError::UnterminatedString { 
                line: self.line,
                start: self.start,
                end: self.curr
            });
        }

        self.advance()?;

        let value: String = String::from(&self.source[self.start + 1..self.curr - 1]);
        self.add_string_token(TokenType::String, value);
        Ok(())
    }

    /// Processes numbers when a digit is found, and, similarly to `string()`, it repeatedly 
    /// advances as long as the next character is a digit or is not a decimal point followed by the
    /// fractional part of the number
    /// 
    /// ## Returns
    /// [`Result<(), LexerError>`]
    fn number(&mut self) -> Result<(), LexerError> {
        while self.is_digit(self.peek()?) {
            self.advance()?;
        }

        if self.peek()? == '.' && self.is_digit(self.peek_next()?) {
            self.advance()?;

            while self.is_digit(self.peek()?) {
                self.advance()?;
            }
        }

        let value = String::from(&self.source[self.start..self.curr]);
        self.add_string_token(TokenType::Num, value);
        Ok(())
    }

    fn identifier(&mut self) -> Result<(), LexerError> {
        while !self.is_at_end() && self.is_alpha(self.peek()?) {
            self.advance()?;
        }

        let text = String::from(&self.source[self.start..self.curr]);
        let token_type: TokenType = match self.keywords.get(&text) {
            Some(v) => *v,
            None => TokenType::Identifier,
        };

        self.add_token(token_type);
        Ok(())
    }

    /// Scans the source code for the next token, and adds it to the tokens vector.
    fn scan_token(&mut self) -> Result<(), LexerError> {
        // Gets the next character of the source code
        let c = self.advance()?;
        let token: TokenType;

        self.handle_indents()?;

        // Matches the character to a token, and moves along the source code if necessary in the
        // case of tokens like identifiers and literals. It also advances for double-character
        // operators so they are properly processed
        match c {
            '(' => token = TokenType::LParen,
            ')' => token = TokenType::RParen,
            '{' => token = TokenType::LBrace,
            '}' => token = TokenType::RBrace,
            '[' => token = TokenType::LBrack,
            ']' => token = TokenType::RBrack,
            ',' => token = TokenType::Comma,
            ';' => token = TokenType::Semicolon,
            ':' => token = TokenType::Colon,
            '*' => token = TokenType::Asterisk,
            '.' => {
                if self.match_token('.') {
                    token = TokenType::DotDot;
                } else {
                    token = TokenType::Dot;
                }
            },
            '-' => {
                if self.match_token('-') {
                    token = TokenType::Decr;
                } else {
                    token = TokenType::Minus;
                }
            }
            '+' => {
                if self.match_token('+') {
                    token = TokenType::Incr;
                } else {
                    token = TokenType::Plus;
                }
            }
            '!' => {
                if self.match_token('=') {
                    token = TokenType::BangEqual;
                } else {
                    token = TokenType::Bang;
                }
            }
            '=' => {
                if self.match_token('=') {
                    token = TokenType::EqualEqual;
                } else {
                    token = TokenType::Equal;
                }
            }
            '<' => {
                if self.match_token('=') {
                    token = TokenType::LessEqual;
                } else {
                    token = TokenType::Less;
                }
            }
            '>' => {
                if self.match_token('=') {
                    token = TokenType::GreaterEqual;
                } else {
                    token = TokenType::Greater;
                }
            }
            '\r' => {
                while self.match_token('\n') {}
                if self.is_at_end() {
                    return Ok(());
                }
                self.line += 1;
                self.is_new_line = true;
                return self.handle_indents();
            }
            '\n' => {
                self.line += 1;
                self.is_new_line = true;
                return self.handle_indents();
            }
            ' ' | '\t' => return Ok(()),
            '/' => {
                if self.match_token('/') {
                    while self.peek()? != '\n' && !self.is_at_end() {
                        self.advance()?;
                    }
                } else {
                    self.add_token(TokenType::FSlash);
                }
                return Ok(());
            }
            '"' => {
                return match self.string() {
                    Err(e) => Err(e),
                    Ok(()) => Ok(()),
                };
            }
            _ => {
                if self.is_digit(c) {
                    self.number()?;
                } else if self.is_alpha(c) {
                    self.identifier()?;
                } else {
                    return Err(LexerError::UnexpectedCharacter {
                        c,
                        line: self.line,
                        start: self.start,
                        end: self.curr
                    });
                }
                return Ok(());
            }
        }
        self.add_token(token);
        return Ok(());
    }

    fn handle_indents(&mut self) -> Result<(), LexerError> {
        // Checks if there is a tab, which will be at the start of a line
        // From CPython lexer implementation
        // https://github.com/python/cpython/blob/main/Parser/lexer/lexer.c
        // Line 423
        if self.is_new_line {
            let mut col = 0;
            const TABSIZE: i32 = 2;
            loop {
                if self.match_token(' ') {
                    col += 1;
                } else if self.match_token('\t') {
                    col += TABSIZE;
                } else {
                    break;
                }
            }

            let indent_count = if col % TABSIZE == 0 {
                (col / TABSIZE) as usize
            } else {
                println!("Incorrect Indentation Error, Col: {col}");
                return Err(LexerError::IncorrectIndentation { line: self.line });
            };
            // dbg!(&self.source.chars().collect::<Vec<char>>()[self.curr]);
            // dbg!(col);
            // dbg!(indent_count);
            if indent_count > self.indent {
                for _ in 0..indent_count - self.indent {
                    self.tokens.push(Token::new(
                        TokenType::Indent,
                        "".to_string(),
                        "".to_string(),
                        self.line,
                        self.start,
                        self.curr
                    ));
                }
            } else if indent_count < self.indent {
                for _ in 0..self.indent - indent_count {
                    if !self.is_at_end() {
                        self.tokens.push(Token::new(
                            TokenType::Dedent,
                            "".to_string(),
                            "".to_string(),
                            self.line,
                            self.start,
                            self.curr
                        ));
                    }
                }
            }
            if indent_count > 0 {
                self.is_indented = true;
            } else {
                self.is_indented = false;
            }
            self.indent = indent_count;
            self.is_new_line = false;
            return Ok(());
        } else {
            return Ok(());
        }
    }

    /// Advances to the next character in the program and returns it. If there are no more
    /// characters left it will return `LexerError::NoCharactersLeft`
    fn advance(&mut self) -> Result<char, LexerError> {
        return if let Some(c) = self.source.chars().nth(self.curr) {
            self.curr += 1;
            Ok(c)
        } else {
            Err(LexerError::NoCharactersLeft {
                line: self.line,
                start: self.start,
                end: self.curr
            })
        };
    }

    /// Takes a look at the current character in the source code, and returns it if the scanner is
    /// not at the end of the source code, otherwise it will return
    /// `LexerError::CannotPeekAtTheEnd`
    fn peek(&self) -> Result<char, LexerError> {
        if self.is_at_end() {
            return Err(LexerError::CannotPeekAtTheEnd {
                line: self.line,
                start: self.start,
                end: self.curr
            });
        }
        return Ok(self.source.chars().nth(self.curr).unwrap());
    }

    /// Takes a look at the next character in the source code, and returns it if the scanner is not
    /// at the end of the source code, otherwise it will return `LexerError::CannotPeekAtTheEnd`
    fn peek_next(&self) -> Result<char, LexerError> {
        if self.curr + 1 >= self.source.len() {
            return Err(LexerError::NoCharactersLeft {
                line: self.line,
                start: self.start,
                end: self.curr
            });
        }
        return Ok(self.source.chars().nth(self.curr + 1).unwrap());
    }

    /// Checks if the current character in the source code is the expected character, and if it is,
    /// then it will advance and return `true`. If the scanner is at the end of the source code or
    /// the current character is not the expected one, then the function will return false
    /// 
    /// ## Arguments
    /// - `expected`: The `char` that it expects to be the current character
    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        };

        if self.source.chars().nth(self.curr).unwrap() != expected {
            return false;
        };

        self.curr += 1;
        return true;
    }

    fn is_digit(&mut self, c: char) -> bool {
        return c.is_ascii_digit();
    }

    fn is_alpha(&self, c: char) -> bool {
        return c.is_alphanumeric() || (c == '_');
    }

    fn is_at_end(&self) -> bool {
        return self.curr >= self.source.len();
    }
}
