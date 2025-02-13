use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    LParen, RParen, LBrace, RBrace, LBrack, RBrack, Comma, Dot, DotDot,
    Minus, Plus, Semicolon, Colon, FSlash, Asterisk, Incr, Decr,

    Bang, BangEqual, Equal, EqualEqual,
    Greater,GreaterEqual, Less, LessEqual,

    Identifier, String, Num,

    And, Def, Else, False, For, If, In, Let, Null, 
    Or, Print, Return, Step, True, While,

    Eof, Indent, Dedent
}

/// The `Token` struct represents a token in the source code.
///
/// ## Fields
/// - `token_type`: The type of the token.
/// - `lexeme`: The lexeme (text) of the token.
/// - `literal`: The literal value of the token, if any.
/// - `line`: The line number where the token is located.
/// - `start`: The starting index of the token in the source code.
/// - `end`: The ending index of the token in the source code.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
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

impl fmt::Display for TokenType {
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "Token{{{}, {}, {}, {}, {}, {}}}",
            self.token_type, self.lexeme, self.literal, self.line, self.start, self.end,
        );
    }
}