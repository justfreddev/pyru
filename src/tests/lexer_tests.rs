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
fn test_comments() {
    assert_eq!(
        lex("// Comment body"),
        vec![
        ]
    );
}

#[test]
fn test_double_symbols() {
    assert_eq!(
        lex("-- ++ != == <= >= .."),
        vec![
            token!(Decr ; "--" ; "" ; 1 ; 0 ; 2),
            token!(Incr ; "++" ; "" ; 1 ; 3 ; 5),
            token!(BangEqual ; "!=" ; "" ; 1 ; 6 ; 8),
            token!(EqualEqual ; "==" ; "" ; 1 ; 9 ; 11),
            token!(LessEqual ; "<=" ; "" ; 1 ; 12 ; 14),
            token!(GreaterEqual ; ">=" ; "" ; 1 ; 15 ; 17),
            token!(DotDot ; ".." ; "" ; 1 ; 18 ; 20),
            token!(Eof ; "" ; "" ; 1 ; 20 ; 20)
        ]
    );
}

#[test]
fn test_identifiers() {
    assert_eq!(
        lex("identifier;"),
        vec![
            token!(Identifier ; "identifier" ; "" ; 1 ; 0 ; 10),
            token!(Semicolon ; ";" ; "" ; 1 ; 10 ; 11),
            token!(Eof ; "" ; "" ; 1 ; 11 ; 11)
        ]
    );

    assert_eq!(
        lex("identifier_2;"),
        vec![
            token!(Identifier ; "identifier_2" ; "" ; 1 ; 0 ; 12),
            token!(Semicolon ; ";" ; "" ; 1 ; 12 ; 13),
            token!(Eof ; "" ; "" ; 1 ; 13 ; 13)
        ]
    );

    assert_eq!(
        lex("boundary identifier;"),
        vec![
            token!(Identifier ; "boundary" ; "" ; 1 ; 0 ; 8),
            token!(Identifier ; "identifier" ; "" ; 1 ; 9 ; 19),
            token!(Semicolon ; ";" ; "" ; 1 ; 19 ; 20),
            token!(Eof ; "" ; "" ; 1 ; 20 ; 20)
        ]
    );

    assert_eq!(
        lex("erroneous-identifier;"),
        vec![
            token!(Identifier ; "erroneous" ; "" ; 1 ; 0 ; 9),
            token!(Minus ; "-" ; "" ; 1 ; 9 ; 10),
            token!(Identifier ; "identifier" ; "" ; 1 ; 10 ; 20),
            token!(Semicolon ; ";" ; "" ; 1 ; 20 ; 21),
            token!(Eof ; "" ; "" ; 1 ; 21 ; 21)
        ]
    );
}

#[test]
fn test_keywords() {
    assert_eq!(
        lex(
            "and def else false for if let not null or print return step true while"
        ),
        vec![
            token!(And ; "and" ; "" ; 1 ; 0 ; 3),
            token!(Def ; "def" ; "" ; 1 ; 4 ; 7),
            token!(Else ; "else" ; "" ; 1 ; 8 ; 12),
            token!(False ; "false" ; "" ; 1 ; 13 ; 18),
            token!(For ; "for" ; "" ; 1 ; 19 ; 22),
            token!(If ; "if" ; "" ; 1 ; 23 ; 25),
            token!(Let ; "let" ; "" ; 1 ; 26 ; 29),
            token!(Not ; "not" ; "" ; 1 ; 30 ; 33),
            token!(Null ; "null" ; "" ; 1 ; 34 ; 38),
            token!(Or ; "or" ; "" ; 1 ; 39 ; 41),
            token!(Print ; "print" ; "" ; 1 ; 42 ; 47),
            token!(Return ; "return" ; "" ; 1 ; 48 ; 54),
            token!(Step ; "step" ; "" ; 1 ; 55 ; 59),
            token!(True ; "true" ; "" ; 1 ; 60 ; 64),
            token!(While ; "while" ; "" ; 1 ; 65 ; 70),
            token!(Eof ; "" ; "" ; 1 ; 70 ; 70)
        ]
    );
}

#[test]
fn test_new_lines() {
    assert_eq!(
        lex("print(\n123\n);"),
        vec![
            token!(Print ; "print" ; "" ; 1 ; 0 ; 5),
            token!(LParen ; "(" ; "" ; 1 ; 5 ; 6),
            token!(Num ; "123" ; "123" ; 2 ; 7 ; 10),
            token!(RParen ; ")" ; "" ; 3 ; 11 ; 12),
            token!(Semicolon ; ";" ; "" ; 3 ; 12 ; 13),
            token!(Eof ; "" ; "" ; 3 ; 13 ; 13)
        ]
    );

    assert_eq!(
        lex("print(\r\n123\r\n);"),
        vec![
            token!(Print ; "print" ; "" ; 1 ; 0 ; 5),
            token!(LParen ; "(" ; "" ; 1 ; 5 ; 6),
            token!(Num ; "123" ; "123" ; 2 ; 8 ; 11),
            token!(RParen ; ")" ; "" ; 3 ; 13 ; 14),
            token!(Semicolon ; ";" ; "" ; 3 ; 14 ; 15),
            token!(Eof ; "" ; "" ; 3 ; 15 ; 15)
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
        lex("print(123);"),
        vec![
            token!(Print ; "print" ; "" ; 1 ; 0 ; 5),
            token!(LParen ; "(" ; "" ; 1 ; 5 ; 6),
            token!(Num ; "123" ; "123" ; 1 ; 6 ; 9),
            token!(RParen ; ")" ; "" ; 1 ; 9 ; 10),
            token!(Semicolon ; ";" ; "" ; 1 ; 10 ; 11),
            token!(Eof ; "" ; "" ; 1 ; 11 ; 11),
        ]
    );

    assert_eq!(
        lex("print(0);"),
        vec![
            token!(Print ; "print" ; "" ; 1 ; 0 ; 5),
            token!(LParen ; "(" ; "" ; 1 ; 5 ; 6),
            token!(Num ; "0" ; "0" ; 1 ; 6 ; 7),
            token!(RParen ; ")" ; "" ; 1 ; 7 ; 8),
            token!(Semicolon ; ";" ; "" ; 1 ; 8 ; 9),
            token!(Eof ; "" ; "" ; 1 ; 9 ; 9),
        ]
    );

    assert_eq!(
        lex("print(-0);"),
        vec![
            token!(Print ; "print" ; "" ; 1 ; 0 ; 5),
            token!(LParen ; "(" ; "" ; 1 ; 5 ; 6),
            token!(Minus ; "-" ; "" ; 1 ; 6 ; 7),
            token!(Num ; "0" ; "0" ; 1 ; 7 ; 8),
            token!(RParen ; ")" ; "" ; 1 ; 8 ; 9),
            token!(Semicolon ; ";" ; "" ; 1 ; 9 ; 10),
            token!(Eof ; "" ; "" ; 1 ; 10 ; 10),
        ]
    );

    assert_eq!(
        lex("print(123.456);"),
        vec![
            token!(Print ; "print" ; "" ; 1 ; 0 ; 5),
            token!(LParen ; "(" ; "" ; 1 ; 5 ; 6),
            token!(Num ; "123.456" ; "123.456" ; 1 ; 6 ; 13),
            token!(RParen ; ")" ; "" ; 1 ; 13 ; 14),
            token!(Semicolon ; ";" ; "" ; 1 ; 14 ; 15),
            token!(Eof ; "" ; "" ; 1 ; 15 ; 15),
        ]
    );

    assert_eq!(
        lex("print(-0.001);"),
        vec![
            token!(Print ; "print" ; "" ; 1 ; 0 ; 5),
            token!(LParen ; "(" ; "" ; 1 ; 5 ; 6),
            token!(Minus ; "-" ; "" ; 1 ; 6 ; 7),
            token!(Num ; "0.001" ; "0.001" ; 1 ; 7 ; 12),
            token!(RParen ; ")" ; "" ; 1 ; 12 ; 13),
            token!(Semicolon ; ";" ; "" ; 1 ; 13 ; 14),
            token!(Eof ; "" ; "" ; 1 ; 14 ; 14),
        ]
    );
}

#[test]
fn test_single_symbols() {
    assert_eq!(
        lex("( ) { } , . : ; * / - + ! = < >"),
        vec![
            token!(LParen ; "(" ; "" ; 1 ; 0 ; 1),
            token!(RParen ; ")" ; "" ; 1 ; 2 ; 3),
            token!(LBrace ; "{" ; "" ; 1 ; 4 ; 5),
            token!(RBrace ; "}" ; "" ; 1 ; 6 ; 7),
            token!(Comma ; "," ; "" ; 1 ; 8 ; 9),
            token!(Dot ; "." ; "" ; 1 ; 10 ; 11),
            token!(Colon ; ":" ; "" ; 1 ; 12 ; 13),
            token!(Semicolon ; ";" ; "" ; 1 ; 14 ; 15),
            token!(Asterisk ; "*" ; "" ; 1 ; 16 ; 17),
            token!(FSlash ; "/" ; "" ; 1 ; 18 ; 19),
            token!(Minus ; "-" ; "" ; 1 ; 20 ; 21),
            token!(Plus ; "+" ; "" ; 1 ; 22 ; 23),
            token!(Bang ; "!" ; "" ; 1 ; 24 ; 25),
            token!(Equal ; "=" ; "" ; 1 ; 26 ; 27),
            token!(Less ; "<" ; "" ; 1 ; 28 ; 29),
            token!(Greater ; ">" ; "" ; 1 ; 30 ; 31),
            token!(Eof ; "" ; "" ; 1 ; 31 ; 31)
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