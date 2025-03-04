use crate::{
    lexer::Lexer,
    token::Token,
    token::TokenType,
};

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

fn lex(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source.to_string(), 4);
    let tokens = match lexer.run() {
        Ok(t) => {
            t
        },
        Err(e) => {
            eprintln!("{e}");
            Vec::new()
        }
    };
    return tokens;
}


#[test]
fn test_single_symbols() {
    assert_eq!(
        lex("( ) { } , . ; * / - + ! = < >"),
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
        lex("-- ++ != == <= >="),
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
        lex("\"string\";"),
        vec![
            token!(String ; "\"string\"" ; "string" ; 1 ; 0 ; 8),
            token!(Semicolon ; ";" ; "" ; 1 ; 8 ; 9),
            token!(Eof ; "" ; "" ; 1 ; 9 ; 9)
        ]
    );

    assert_eq!(
        lex("\"Unterminated"),
        vec![]
    );

    assert_eq!(
        lex("\"New\n\rline\";"),
        vec![]
    );
}

#[test]
fn test_comments() {
    assert_eq!(
        lex("// Comment body"),
        vec![
        ]
    );
}

#[test]
fn test_keywords() {
    assert_eq!(
        lex(
            "and def else false for if null or print return step true let while"
        ),
        vec![
            token!(And ; "and" ; "" ; 1 ; 0 ; 3),
            token!(Def ; "def" ; "" ; 1 ; 4 ; 7),
            token!(Else ; "else" ; "" ; 1 ; 8 ; 12),
            token!(False ; "false" ; "" ; 1 ; 13 ; 18),
            token!(For ; "for" ; "" ; 1 ; 19 ; 22),
            token!(If ; "if" ; "" ; 1 ; 23 ; 25),
            token!(Null ; "null" ; "" ; 1 ; 26 ; 30),
            token!(Or ; "or" ; "" ; 1 ; 31 ; 33),
            token!(Print ; "print" ; "" ; 1 ; 34 ; 39),
            token!(Return ; "return" ; "" ; 1 ; 40 ; 46),
            token!(Step ; "step" ; "" ; 1 ; 47 ; 51),
            token!(True ; "true" ; "" ; 1 ; 52 ; 56),
            token!(Let ; "let" ; "" ; 1 ; 57 ; 60),
            token!(While ; "while" ; "" ; 1 ; 61 ; 66),
            token!(Eof ; "" ; "" ; 1 ; 66 ; 66)
        ]
    );
}

#[test]
fn test_nums() {
    assert_eq!(
        lex("123."),
        Vec::new()
    );

    assert_eq!(
        lex(".123;"),
        vec![
            token!(Dot ; "." ; "" ; 1 ; 0 ; 1),
            token!(Num ; "123" ; "123" ; 1 ; 1 ; 4),
            token!(Semicolon ; ";" ; "" ; 1 ; 4 ; 5),
            token!(Eof ; "" ; "" ; 1 ; 5; 5),
        ]
    );

    assert_eq!(
        lex("print 123;"),
        vec![
            token!(Print ; "print" ; "" ; 1 ; 0 ; 5),
            token!(Num ; "123" ; "123" ; 1 ; 6 ; 9),
            token!(Semicolon ; ";" ; "" ; 1 ; 9 ; 10),
            token!(Eof ; "" ; "" ; 1 ; 10 ; 10),
        ]
    );

    assert_eq!(
        lex("print 0;"),
        vec![
            token!(Print ; "print" ; "" ; 1 ; 0 ; 5),
            token!(Num ; "0" ; "0" ; 1 ; 6 ; 7),
            token!(Semicolon ; ";" ; "" ; 1 ; 7 ; 8),
            token!(Eof ; "" ; "" ; 1 ; 8 ; 8),
        ]
    );

    assert_eq!(
        lex("print -0;"),
        vec![
            token!(Print ; "print" ; "" ; 1 ; 0 ; 5),
            token!(Minus ; "-" ; "" ; 1 ; 6 ; 7),
            token!(Num ; "0" ; "0" ; 1 ; 7 ; 8),
            token!(Semicolon ; ";" ; "" ; 1 ; 8 ; 9),
            token!(Eof ; "" ; "" ; 1 ; 9 ; 9),
        ]
    );

    assert_eq!(
        lex("print 123.456;"),
        vec![
            token!(Print ; "print" ; "" ; 1 ; 0 ; 5),
            token!(Num ; "123.456" ; "123.456" ; 1 ; 6 ; 13),
            token!(Semicolon ; ";" ; "" ; 1 ; 13 ; 14),
            token!(Eof ; "" ; "" ; 1 ; 14 ; 14),
        ]
    );

    assert_eq!(
        lex("print -0.001;"),
        vec![
            token!(Print ; "print" ; "" ; 1 ; 0 ; 5),
            token!(Minus ; "-" ; "" ; 1 ; 6 ; 7),
            token!(Num ; "0.001" ; "0.001" ; 1 ; 7 ; 12),
            token!(Semicolon ; ";" ; "" ; 1 ; 12 ; 13),
            token!(Eof ; "" ; "" ; 1 ; 13 ; 13),
        ]
    );
}