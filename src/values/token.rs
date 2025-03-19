//! The `token` module defines the `Token` and `TokenType` structures, which represent the tokens
//! generated by the lexer during the lexical analysis phase. Tokens are the smallest units of
//! meaning in the source code and are used by the parser to construct the abstract syntax tree (AST).
//!
//! ## Overview
//!
//! The `TokenType` enum defines the various types of tokens that can be encountered in the source code,
//! such as parentheses, operators, keywords, literals, and more. Each token type corresponds to a specific
//! syntactic or semantic element of the language.
//!
//! The `Token` struct represents an individual token and contains information about its type, lexeme (text),
//! literal value (if applicable), and its position in the source code (line number and character indices).
//!
//! ## Example
//!
//! ```rust
//! use crate::token::{Token, TokenType};
//!
//! let token = Token::new(
//!     TokenType::Identifier,
//!     "x".to_string(),
//!     "".to_string(),
//!     1,
//!     0,
//!     1,
//! );
//!
//! println!("{}", token);
//! ```
//!
//! ## Usage
//!
//! Tokens are created by the lexer and passed to the parser for further processing. The `TokenType` enum
//! provides a comprehensive list of all possible token types, while the `Token` struct encapsulates the
//! details of each token.

use std::fmt;

/// Represents the different types of tokens that can be encountered in the source code.
/// 
/// ## Variants
/// - `LParen`, `RParen`: Represents `(` and `)` parentheses.
/// - `LBrace`, `RBrace`: Represents `{` and `}` braces.
/// - `LBrack`, `RBrack`: Represents `[` and `]` brackets.
/// - `Comma`, `Dot`, `DotDot`: Represents `,`, `.`, and `..`.
/// - `Minus`, `Plus`, `Semicolon`, `Colon`, `FSlash`, `Asterisk`: Represents `-`, `+`, `;`, `:`, `/`, and `*`.
/// - `Incr`, `Decr`: Represents `++` and `--`.
/// - `Bang`, `BangEqual`: Represents `!` and `!=`.
/// - `Equal`, `EqualEqual`: Represents `=` and `==`.
/// - `Greater`, `GreaterEqual`, `Less`, `LessEqual`: Represents comparison operators.
/// - `Identifier`, `String`, `Num`: Represents identifiers, string literals, and numeric literals.
/// - Keywords: `And`, `Def`, `Else`, `False`, `For`, `If`, `In`, `Let`, `Not`, `Null`, `Or`, `Print`, `Return`, `Step`, `True`, `While`.
/// - `Eof`: Represents the end of the file.
/// - `Indent`, `Dedent`: Represents changes in indentation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    LParen, RParen, LBrace, RBrace, LBrack, RBrack, Comma, Dot, DotDot,
    Minus, Plus, Semicolon, Colon, FSlash, Asterisk, Incr, Decr,

    Bang, BangEqual, Equal, EqualEqual,
    Greater, GreaterEqual, Less, LessEqual,

    Identifier, String, Num,

    And, Def, Else, False, For, If, In, Let, Not,
    Null, Or, Print, Return, Step, True, While,

    Eof, Indent, Dedent
}

/// Represents a token in the source code.
///
/// ## Fields
/// - `token_type`: The type of the token (e.g., `Identifier`, `String`, `Num`).
/// - `lexeme`: The lexeme (text) of the token.
/// - `literal`: The literal value of the token, if any (e.g., the value of a string or number).
/// - `line`: The line number where the token is located.
/// - `start`: The starting index of the token in the source code.
/// - `end`: The ending index of the token in the source code.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType, // ENUM COMPOSITION - BAND A EQUIVALENT
    pub lexeme: String,
    pub literal: String,
    pub line: usize,
    pub start: usize,
    pub end: usize
}

impl Token {
    /// Creates a new `Token` instance.
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: usize, start: usize, end: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
            start,
            end
        }
    }
}

impl fmt::Display for TokenType { // INTERFACES - BAND A
    /// Implements the `Display` trait for `TokenType` to provide a string representation
    /// of each token type.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::LParen => write!(f, "LParen"),
            TokenType::RParen => write!(f, "RParen"),
            TokenType::LBrace => write!(f, "LBrace"),
            TokenType::RBrace => write!(f, "RBrace"),
            TokenType::LBrack => write!(f, "LBrack"),
            TokenType::RBrack => write!(f, "RBrack"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::Dot => write!(f, "Dot"),
            TokenType::DotDot => write!(f, "DotDot"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Colon => write!(f, "Colon"),
            TokenType::FSlash => write!(f, "FSlash"),
            TokenType::Asterisk => write!(f, "Asterisk"),
            TokenType::Incr => write!(f, "Incr"),
            TokenType::Decr => write!(f, "Decr"),
            TokenType::Bang => write!(f, "Bang"),
            TokenType::BangEqual => write!(f, "BangEqual"),
            TokenType::Equal => write!(f, "Equal"),
            TokenType::EqualEqual => write!(f, "EqualEqual"),
            TokenType::Greater => write!(f, "Greater"),
            TokenType::GreaterEqual => write!(f, "GreaterEqual"),
            TokenType::Less => write!(f, "Less"),
            TokenType::LessEqual => write!(f, "LessEqual"),
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::String => write!(f, "String"),
            TokenType::Num => write!(f, "Num"),
            TokenType::And => write!(f, "And"),
            TokenType::Else => write!(f, "Else"),
            TokenType::False => write!(f, "False"),
            TokenType::For => write!(f, "For"),
            TokenType::Def => write!(f, "Def"),
            TokenType::If => write!(f, "If"),
            TokenType::In => write!(f, "In"),
            TokenType::Let => write!(f, "Let"),
            TokenType::Not => write!(f, "Not"),
            TokenType::Null => write!(f, "Null"),
            TokenType::Or => write!(f, "Or"),
            TokenType::Print => write!(f, "Print"),
            TokenType::Return => write!(f, "Return"),
            TokenType::Step => write!(f, "Step"),
            TokenType::True => write!(f, "True"),
            TokenType::While => write!(f, "While"),
            TokenType::Eof => write!(f, "Eof"),
            TokenType::Indent => write!(f, "Indent"),
            TokenType::Dedent => write!(f, "Dedent"),
        }
    }
}

impl fmt::Display for Token { // INTERFACES - BAND A
    /// Implements the `Display` trait for `Token` to provide a string representation
    /// of the token, including its type, lexeme, literal, and position.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "Token{{{}, {}, {}, {}, {}, {}}}",
            self.token_type, self.lexeme, self.literal, self.line, self.start, self.end,
        );
    }
}