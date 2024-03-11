use std::fmt;

#[derive(Clone, Debug)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace, Comma,
    Dot, Minus, Plus, Semicolon, FSlash, Asterisk,

    Bang, BangEqual, Equal, EqualEqual,
    Greater,GreaterEqual, Less, LessEqual,

    Identifier, String, Num,

    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    _literal: String,
    _line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            _literal: literal,
            _line: line,
        } 
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}({})", self.token_type, self.lexeme)
    }
}