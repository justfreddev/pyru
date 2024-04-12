use crate::{
    lexer::Lexer,
    token::Token,
    token::TokenType
};

// The lexers whole job is just to tokenise source code, so it shouldn't be too difficult
// to test, it will just take a long to test

#[macro_export]
macro_rules! token {
    ($token:ident ; $lexeme:literal ; $lit:literal ; $line:literal ; $start:literal ; $end:literal) => {
        Token {
            token_type: TokenType::$token,
            lexeme: $lexeme.to_string(),
            literal: $lit.to_string(),
            line: $line,
            start: $start,
            end: $end
        }
    };
}

fn lex(source: String) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.run() {
        Ok(t) => t,
        Err(_) => Vec::new()
    };
    return tokens;
}


#[test]
fn test_single_symbols() {
    assert_eq!(
        lex("( ) { } , . ; * / - + ! = < >".to_string()),
        vec![
            token!(LParen ; "(" ; "" ; 1 ; 0 ; 1),
            token!(RParen ; ")" ; "" ; 1 ; 2 ; 3),
            token!(LBrace ; "{" ; "" ; 1 ; 4 ; 5),
            token!(RBrace ; "}" ; "" ; 1 ; 6 ; 7),
            token!(Comma ; "," ; "" ; 1 ; 8 ; 9),
            token!(Dot ; "." ; "" ; 1 ; 10 ; 11),
            token!(Semicolon ; ";" ; "" ; 1 ; 12 ; 13),
            token!(Asterisk ; "*" ; "" ; 1 ; 14 ; 15),
            token!(FSlash ; "/" ; "" ; 1 ; 16 ; 17),
            token!(Minus ; "-" ; "" ; 1 ; 18 ; 19),
            token!(Plus ; "+" ; "" ; 1 ; 20 ; 21),
            token!(Bang ; "!" ; "" ; 1 ; 22 ; 23),
            token!(Equal ; "=" ; "" ; 1 ; 24 ; 25),
            token!(Less ; "<" ; "" ; 1 ; 26 ; 27),
            token!(Greater ; ">" ; "" ; 1 ; 28 ; 29),
            token!(Eof ; "" ; "" ; 1 ; 29 ; 29)
        ]
    );
}

#[test]
fn test_double_symbols() {
    assert_eq!(
        lex("-- ++ != == <= >=".to_string()),
        vec![
            token!(Decr ; "--" ; "" ; 1 ; 0 ; 2),
            token!(Incr ; "++" ; "" ; 1 ; 3 ; 5),
            token!(BangEqual ; "!=" ; "" ; 1 ; 6 ; 8),
            token!(EqualEqual ; "==" ; "" ; 1 ; 9 ; 11),
            token!(LessEqual ; "<=" ; "" ; 1 ; 12 ; 14),
            token!(GreaterEqual ; ">=" ; "" ; 1 ; 15 ; 17),
            token!(Eof ; "" ; "" ; 1 ; 17 ; 17),
        ]
    );
}

#[test]
fn test_strings() {
    assert_eq!(
        lex("\"string\"".to_string()),
        vec![
            token!(String ; "\"string\"" ; "string" ; 1 ; 0 ; 8),
            token!(Eof ; "" ; "" ; 1 ; 8 ; 8)
        ]
    );

    assert_eq!(
        lex("\"Unterminated".to_string()),
        vec![]
    );

    assert_eq!(
        lex("\"New\n\rline\"".to_string()),
        vec![
            token!(String ; "\"New\n\rline\"" ; "New\n\rline" ; 2 ; 0 ; 11),
            token!(Eof ; "" ; "" ; 2 ; 11 ; 11),
        ]
    );
}