use std::{fmt, io::Write};


#[derive(Debug)]
enum TokenType {
    LeftParen,
    RightParen,
    Minus,
    Plus,
    Semicolon,
    FSlash,
    Asterisk,

    Num,

    Eof,
}



struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}


struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    curr: usize,
    line: usize,
}


impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: String, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        } 
    }

}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {}", self.token_type, self.lexeme)
    }
}


impl Lexer {
    fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            curr: 0,
            line: 1,
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.curr).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.curr >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.curr += 1;
        self.source.chars().nth(self.curr).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = String::from(&self.source[self.start..self.curr]);
        self.tokens.push(Token::new(token_type, text, String::from(""), self.line))
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        self.add_token(match c {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Asterisk,
            _ => TokenType::Eof
        })
    }

    fn scanToken(&mut self) {
        while !self.is_at_end() {
            let start = self.curr;
            self.scan_token();
        };

        self.tokens.push(Token::new(TokenType::Eof, String::from(""), String::from(""), self.line));
    }
}


fn main() {
    let mut inp = String::new();

    print!("> ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut inp).unwrap();

    let mut lexemes: Vec<char> = inp.chars().collect();
    lexemes.retain(|c| !c.is_whitespace());

    for c in lexemes {

    }

}