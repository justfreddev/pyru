//! The parser module is responsible for parsing the tokens generated by the lexer and constructing
//! an abstract syntax tree (AST) for the evaluator. The parser analyses the structure and
//! grammar of the source code to determine its meaning and validity. The parser follows the rules
//! defined by the programming language's grammar to ensure that the code is syntactically correct.
//! If any syntax errors are encountered during parsing, the parser reports them as
//! `ParserError`, which is then returned as an Err(), which safely halts the program. Once the
//! parsing process is complete, the parser returns the AST, which is then used by the evaluator
//! to execute the program.
//!
//! This module defines the Parser struct, which maintains the state of the parsing process.
//! It contains methods for parsing different language features such as declarations,
//! expressions, statements, loops, conditionals, and more. The parse() method is the entry
//! point of the parser, which starts the parsing process and returns the resulting AST.
//! The Parser struct also keeps track of the current token being parsed and provides
//! utility methods for token matching, consuming tokens, and error handling.
//! 
//! 
//! ## Example
//! 
//! ```rust
//! use crate::parser::Parser;
//! use crate::lexer::Lexer;
//!
//! fn main() {
//!     let source_code = r#"
//!         var x = 10;
//!         if (x > 5) {
//!             print "Hello, world!";
//!         }
//!     "#;
//!
//!     let mut lexer = Lexer::new(source_code);
//!     let tokens = lexer.run().unwrap();
//!
//!     let mut parser = Parser::new(tokens);
//!     let ast = parser.parse().unwrap();
//!
//!     // Use the AST to execute the program
//!     // ...
//! }
//! ```
//! 
//! ## The Process
//! 
//! Parsing the tokens works as follows:
//! 
//! 1. The parser evaluates the tokens one by one, and starts off by
//!    calling the `parse` method, which initializes the parsing process.
//! 2. The `parse` method repeatedly calls the `declaration` method to parse
//!    top-level declarations until all tokens are consumed.
//! 3. The `declaration` method determines the type of declaration (e.g., function,
//!    variable) and delegates to the appropriate method (`function` which deals with
//!    functions, or `var_declaration` to begin the recursive descent tree traversal).
//! 4. Each specific parsing method (e.g., `function`, `var_declaration`) consumes
//!    tokens and constructs the corresponding AST nodes, handling any syntax errors
//!    encountered along the way.
//! 5. Expressions and statements within declarations are parsed using recursive-descent
//!    methods such as `expression`, `statement`, `assignment`, `term`, `factor`, etc.
//! 6. The parser uses utility methods like `match_token`, `check`, `advance`, and `consume`
//!    to navigate through the tokens and ensure they match the expected grammar rules.
//! 7. If a syntax error is detected, the parser reports it and attempts to recover using
//!    the `synchronize` method, which skips tokens until it finds a suitable point to resume parsing.
//! 8. Once all tokens are processed, the `parse` method returns the constructed AST, which
//!    represents the hierarchical structure of the source code and is used by the evaluator
//!    to execute the program.

use crate::{
    error::ParserError,
    expr::Expr,
    stmt::Stmt,
    token::{Token, TokenType},
    value::LiteralType,
};

/// The `Parser` struct is responsible for parsing tokens generated by the lexer and constructing
/// an abstract syntax tree (AST) for the evaluator, by maintaining the state of the parsing process.
/// 
/// ## Fields
/// 
/// - `tokens`: The list of tokens that are iterated over
/// - `current`: A pointer referencing the current token in the tokens vector
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    
    /// Creates a new `Parser` instance with the given tokens.
    ///
    /// # Parameters
    /// - `tokens`: A vector of `Token` objects to be parsed.
    ///
    /// # Returns
    /// A new `Parser` instance.
    pub fn new(tokens: Vec<Token>) -> Self {
        return Self { tokens, current: 0 };
    }

    /// Starts the parsing process and returns the resulting AST.
    ///
    /// # Returns
    /// A `Result` containing a vector of `Stmt` objects representing the AST if successful,
    /// or a `ParserError` if a syntax error is encountered.
    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e),
            }
        }

        return Ok(statements);
    }

    /// Parses a declaration, which can be a function or variable declaration, or a statement.
    ///
    /// # Returns
    /// A `Result` containing a `Stmt` object if successful, or a `ParserError` if a syntax error is encountered.
    fn declaration(&mut self) -> Result<Stmt, ParserError> {
        if self.match_token(vec![&TokenType::Def]) {
            return match self.function("function") {
                Ok(v) => Ok(v),
                Err(e) => {
                    self.synchronize();
                    Err(e)
                }
            }
        } else if self.match_token(vec![&TokenType::Let]) {
            return match self.var_declaration() {
                Ok(v) => Ok(v),
                Err(e) => {
                    self.synchronize();
                    Err(e)
                }
            }
        } else {
            return self.statement();
        }
    }

    /// Parses a function declaration.
    ///
    /// # Parameters
    /// - `kind`: A string representing the kind of function (e.g., "function").
    fn function(&mut self, kind: &str) -> Result<Stmt, ParserError> {
        let name = match self.consume(
            TokenType::Identifier,
            format!(
                "Expected{}Name",
                kind.chars()
                    .next()
                    .unwrap()
                    .to_uppercase()
                    .collect::<String>()
                    + &kind[1..]
            )
            .as_str(),
        ) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        self.consume(
            TokenType::LParen,
            format!(
                "ExpectedLParenAfter{}Name",
                kind.chars()
                    .next()
                    .unwrap()
                    .to_uppercase()
                    .collect::<String>()
                    + &kind[1..]
            )
            .as_str(),
        )?;

        let mut params: Vec<Token> = Vec::new();
        if !self.check(TokenType::RParen) {
            loop {
                if params.len() >= 255 {
                    let token = self.peek();
                    return Err(ParserError::TooManyParameters {
                        name: name.lexeme,
                        line: token.line,
                    });
                }

                let parameter = self.consume(TokenType::Identifier, "ExpectedParameterName")?;
                params.push(parameter);

                if !self.match_token(vec![&TokenType::Comma]) {
                    break;
                };
            }
        }

        self.consume(TokenType::RParen, "ExpectedRParenAfterParameters")?;

        self.consume(TokenType::Colon, "TODO: Expected colon after parameters")?;

        self.consume(TokenType::Indent, "TODO: Expected function body")?;

        let body = self.body()?;

        return Ok(Stmt::Function { name, params, body });
    }

    /// Begins the recursive descent with parsing a variable declaration
    /// 
    /// # Returns
    /// A `Result` containing a `Stmt` object representing the function if successful,
    /// or a `ParserError` if a syntax error is encountered.
    fn var_declaration(&mut self) -> Result<Stmt, ParserError> {
        let name = self.consume(TokenType::Identifier, "ExpectedVariableName")?;

        let initializer = if self.match_token(vec![&TokenType::Equal]) {
            let expr = self.expression()?;
            Some(expr)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "ExpectedSemicolonAfterVariableDeclaration")?;

        return Ok(Stmt::Var { name, initializer });
    }

    /// Parses a statement, which can be a for, if, print, return, while, or expression statement.
    ///
    /// # Returns
    /// A `Result` containing a `Stmt` object if successful, or a `ParserError` if a syntax error is encountered.
    fn statement(&mut self) -> Result<Stmt, ParserError> {
        if self.match_token(vec![&TokenType::For]) {
            return self.for_statement();
        };
        if self.match_token(vec![&TokenType::If]) {
            return self.if_statement();
        };
        if self.match_token(vec![&TokenType::Print]) {
            return self.print_statement();
        };
        if self.match_token(vec![&TokenType::Return]) {
            return self.return_statement();
        };
        if self.match_token(vec![&TokenType::While]) {
            return self.while_statement();
        };

        return self.expression_statement();
    }

    /// Parses a for statement.
    ///
    /// # Returns
    /// A `Result` containing a `Stmt` object if successful, or a `ParserError` if a syntax error is encountered.
    fn for_statement(&mut self) -> Result<Stmt, ParserError> {

        let name = self.consume(TokenType::Identifier, "ExpectedInitializer")?;

        self.consume(TokenType::In, "ExpectedInAfterIdentifier")?;

        let start = self.expression()?;

        self.consume(TokenType::DotDot, "ExpectedDotDot")?;

        let end = self.expression()?;

        let step = if self.match_token(vec![&TokenType::Step]) {
            let value = self.expression()?;
            Expr::Assign {
                name: name.clone(),
                value: Box::new(Expr::Binary {
                    left: Box::new(Expr::Var { name: name.clone() }),
                    operator: Token::new(
                        TokenType::Plus,
                        "+".to_string(),
                        "".to_string(),
                        0,
                        0,
                        0,
                    ),
                    right: Box::new(value)
                })
            }
            
        } else {
            Expr::Alteration {
                name: name.clone(),
                alteration_type: TokenType::Incr,
            }
        };


        self.consume(TokenType::Colon, "ExpectedColon")?;
        
        self.consume(TokenType::Indent, "ExpectedForBody")?;

        let initializer = Stmt::Var { name: name.clone(), initializer: Some(start) };

        let condition = Expr::Binary {
            left: Box::new(Expr::Var { name: name.clone() }),
            operator: Token::new(
                TokenType::Less,
                "<".to_string(),
                "".to_string(),
                0,
                0,
                0,
            ),
            right: Box::new(end),
        };

        let body = self.body()?;

        return Ok(Stmt::For {
            initializer: Box::new(initializer),
            condition,
            step,
            body,
        });
    }

    /// Parses an if statement.
    ///
    /// # Returns
    /// A `Result` containing a `Stmt` object if successful, or a `ParserError` if a syntax error is encountered.
    fn if_statement(&mut self) -> Result<Stmt, ParserError> {
        let condition = self.expression()?;

        self.consume(TokenType::Colon, "ExpectedColon")?;

        self.consume(TokenType::Indent, "ExpectedIfBody")?;
        
        let then_branch = self.body()?;
        
        let mut else_branch = None;
        if self.match_token(vec![&TokenType::Else]) {
            if self.match_token(vec![&TokenType::Colon]) {
                self.consume(TokenType::Indent, "ExpectedIfBody")?;
                let result = self.statement()?;
                else_branch = Some(Box::new(result));
                self.consume(TokenType::Dedent, "ExpectDedentAfterStmt")?;
            } else {
                let result = self.statement()?;
                else_branch = Some(Box::new(result));
            }
        };

        return Ok(Stmt::If {
            condition,
            then_branch: then_branch,
            else_branch,
        });
    }

    /// Parses a print statement.
    ///
    /// # Returns
    /// A `Result` containing a `Stmt` object if successful, or a `ParserError` if a syntax error is encountered.
    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LParen, "ExpectedLParenBeforePrintValue")?;
        let value = self.expression()?;
        self.consume(TokenType::RParen, "ExpectedRParenAfterPrintValue")?;
        self.consume(TokenType::Semicolon, "ExpectedSemicolonAfterPrint")?;

        return Ok(Stmt::Print { expression: value });
    }

    /// Parses a return statement.
    ///
    /// # Returns
    /// A `Result` containing a `Stmt` object if successful, or a `ParserError` if a syntax error is encountered.
    fn return_statement(&mut self) -> Result<Stmt, ParserError> {
        let keyword = self.previous().clone();
        let mut value = None;
        if !self.check(TokenType::Semicolon) {
            value = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "ExpectedSemicolonAfterReturnValue")?;

        return Ok(Stmt::Return { keyword, value });
    }

    /// Parses a while statement.
    ///
    /// # Returns
    /// A `Result` containing a `Stmt` object if successful, or a `ParserError` if a syntax error is encountered.
    fn while_statement(&mut self) -> Result<Stmt, ParserError> {
        let condition = self.expression()?;

        self.consume(TokenType::Colon, "ExpectedColonAfterWhileCondition")?;
        self.consume(TokenType::Indent, "ExpectWhileBody")?;
        
        let body = self.body()?;

        return Ok(Stmt::While { condition, body });
    }

    /// Parses an expression.
    ///
    /// # Returns
    /// A `Result` containing an `Expr` object if successful, or a `ParserError` if a syntax error is encountered.
    fn expression(&mut self) -> Result<Expr, ParserError> {
        return self.assignment();
    }

    /// Parses an assignment expression.
    ///
    /// # Returns
    /// A `Result` containing an `Expr` object if successful, or a `ParserError` if a syntax error is encountered.
    fn assignment(&mut self) -> Result<Expr, ParserError> {
        let expr = self.or()?;

        if self.match_token(vec![&TokenType::Incr, &TokenType::Decr]) {
            match expr {
                Expr::Var { name } => match self.previous().token_type {
                    TokenType::Incr => {
                        return Ok(Expr::Alteration {
                            name,
                            alteration_type: TokenType::Incr,
                        })
                    }
                    TokenType::Decr => {
                        return Ok(Expr::Alteration {
                            name,
                            alteration_type: TokenType::Decr,
                        })
                    }
                    _ => {
                        let token = self.previous();
                        return Err(ParserError::ExpectedAlterationExpression {
                            line: token.start,
                        });
                    }
                },
                _ => {
                    let token = self.previous();
                    return Err(ParserError::InvalidAlterationTarget {
                        target: token.lexeme.clone(),
                        line: token.line,
                    });
                }
            }
        } else if self.match_token(vec![&TokenType::Equal]) {
            let value = self.assignment()?;

            match expr {
                Expr::Var { name } => {
                    return Ok(Expr::Assign {
                        name,
                        value: Box::new(value),
                    })
                }
                _ => {
                    let token = self.previous();
                    return Err(ParserError::InvalidAssignmentTarget {
                        target: token.lexeme.clone(),
                        line: token.line,
                    });
                }
            }
        }

        return Ok(expr);
    }

    /// Parses a logical OR expression.
    ///
    /// # Returns
    /// A `Result` containing an `Expr` object if successful, or a `ParserError` if a syntax error is encountered.
    fn or(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.and()?;

        while self.match_token(vec![&TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        return Ok(expr);
    }

    /// Parses a logical AND expression.
    ///
    /// # Returns
    /// A `Result` containing an `Expr` object if successful, or a `ParserError` if a syntax error is encountered.
    fn and(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.equality()?;

        while self.match_token(vec![&TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        return Ok(expr);
    }

    /// Parses an equality expression.
    ///
    /// # Returns
    /// A `Result` containing an `Expr` object if successful, or a `ParserError` if a syntax error is encountered.
    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.comparison()?;

        while self.match_token(vec![&TokenType::Bang, &TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr.clone()),
                operator,
                right: Box::new(right),
            };
        }

        return Ok(expr);
    }

    /// Parses a comparison expression.
    ///
    /// # Returns
    /// A `Result` containing an `Expr` object if successful, or a `ParserError` if a syntax error is encountered.
    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.term()?;

        while self.match_token(vec![
            &TokenType::Greater,
            &TokenType::GreaterEqual,
            &TokenType::Less,
            &TokenType::LessEqual,
            &TokenType::BangEqual,
            &TokenType::EqualEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr.clone()),
                operator,
                right: Box::new(right),
            };
        }

        return Ok(expr);
    }

    /// Parses a term expression.
    ///
    /// # Returns
    /// A `Result` containing an `Expr` object if successful, or a `ParserError` if a syntax error is encountered.
    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while self.match_token(vec![&TokenType::Minus, &TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr.clone()),
                operator,
                right: Box::new(right),
            };
        }

        return Ok(expr);
    }

    /// Parses a factor expression.
    ///
    /// # Returns
    /// A `Result` containing an `Expr` object if successful, or a `ParserError` if a syntax error is encountered.
    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while self.match_token(vec![&TokenType::FSlash, &TokenType::Asterisk]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr.clone()),
                operator,
                right: Box::new(right),
            };
        }

        return Ok(expr);
    }

    /// Parses a unary expression.
    ///
    /// # Returns
    /// A `Result` containing an `Expr` object if successful, or a `ParserError` if a syntax error is encountered.
    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.match_token(vec![&TokenType::Bang, &TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        return self.call();
    }

    /// Parses a call expression.
    ///
    /// # Returns
    /// A `Result` containing an `Expr` object if successful, or a `ParserError` if a syntax error is encountered.
    fn call(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(vec![&TokenType::LParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(vec![&TokenType::Dot]) {
                let call = self.call()?;
                let name = match expr {
                    Expr::Var { ref name } => name,
                    _ => {
                        let token = self.peek();
                        return Err(ParserError::CanOnlyCallIdentifiers {
                            value: token.lexeme.clone(),
                            line: token.line,
                        })
                    },
                };

                return Ok(Expr::ListMethodCall { object: name.clone(), call: Box::new(call) })
            } else {
                break;
            }
        }

        return Ok(expr);
    }

    /// Finishes parsing a call expression.
    ///
    /// # Returns
    /// A `Result` containing an `Expr` object if successful, or a `ParserError` if a syntax error is encountered.
    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParserError> {
        let mut arguments: Vec<Expr> = Vec::new();

        if !self.check(TokenType::RParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(ParserError::TooManyArguments { callee });
                }
                let expr = self.expression()?;
                arguments.push(expr);
                if !self.match_token(vec![&TokenType::Comma]) {
                    break;
                };
            }
        }

        self.consume(TokenType::RParen, "ExpectedRParenAfterArguments")?;

        return Ok(Expr::Call {
            callee: Box::new(callee),
            arguments,
        });
    }

    /// Parses a primary expression.
    ///
    /// # Returns
    /// A `Result` containing an `Expr` object if successful, or a `ParserError` if a syntax error is encountered.
    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.match_token(vec![&TokenType::True]) {
            return Ok(Expr::Literal {
                value: LiteralType::True,
            });
        };
        if self.match_token(vec![&TokenType::False]) {
            return Ok(Expr::Literal {
                value: LiteralType::False,
            });
        };
        if self.match_token(vec![&TokenType::Null]) {
            return Ok(Expr::Literal {
                value: LiteralType::Null,
            });
        };

        if self.match_token(vec![&TokenType::Num, &TokenType::String]) {
            match self.previous().token_type {
                TokenType::String => {
                    return Ok(Expr::Literal {
                        value: LiteralType::Str(self.previous().literal.clone()),
                    })
                }
                TokenType::Num => {
                    let n = match self.previous().literal.clone().trim().parse() {
                        Ok(v) => v,
                        Err(_) => {
                            let token = self.previous();
                            return Err(ParserError::UnableToParseLiteralToFloat {
                                value: token.lexeme.clone(),
                                line: token.line,
                            });
                        }
                    };
                    return Ok(Expr::Literal {
                        value: LiteralType::Num(n),
                    });
                }
                _ => {
                    let token = self.previous();
                    return Err(ParserError::ExpectedStringOrNumber {
                        value: token.lexeme.clone(),
                        line: token.line,
                    });
                }
            }
        }

        if self.match_token(vec![&TokenType::Identifier]) {
            let name = self.previous().clone();
            let expr = if self.match_token(vec![&TokenType::LBrack]) {
                let mut start: Option<Box<Expr>> = None;
                let mut end: Option<Box<Expr>> = None;
                let mut is_splice = false;
                if self.peek().token_type != TokenType::Colon {
                    start = Some(Box::new(self.expression()?));
                }
                start = if start.is_some() {
                    Some(start.unwrap())
                } else {
                    None
                };
                if self.match_token(vec![&TokenType::Colon]) {
                    is_splice = true;
                    if self.peek().token_type != TokenType::RBrack {
                        end = Some(Box::new(self.expression()?));
                    }
                    end = if end.is_some() {
                        Some(end.unwrap())
                    } else {
                        None
                    };
                }
                self.consume(TokenType::RBrack, "ExpectedRBrackAfterIndex")?;
                Expr::Splice { list: name, is_splice, start, end }
            } else {
                Expr::Var { name: name.clone() }
            };
            return Ok(expr);
        }

        if self.match_token(vec![&TokenType::LParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RParen, "ExpectedRParenAfterExpression")?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }

        if self.match_token(vec![&TokenType::LBrack]) {
            let mut items: Vec<Expr> = Vec::new();
            loop {
                if self.match_token(vec![&TokenType::RBrack]) {
                    break;
                }
                items.push(self.expression()?);
                if !self.match_token(vec![&TokenType::Comma]) {
                    break;
                }
            }

            self.consume(TokenType::RBrack, "ExpectedRBrackAfterValues")?;

            return Ok(Expr::List { items });
        }

        let prev = self.previous();
        let token = self.peek();

        return Err(ParserError::ExpectedExpression {
            prev: prev.lexeme.clone(),
            line: token.line,
        });
    }

    /// Parses an expression statement.
    ///
    /// # Returns
    /// A `Result` containing a `Stmt` object if successful, or a `ParserError` if a syntax error is encountered.
    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        let expr = self.expression()?;

        self.consume(TokenType::Semicolon, "ExpectedExpression")?;

        return Ok(Stmt::Expression { expression: expr });
    }

    /// Parses a block of statements.
    ///
    /// # Returns
    /// A `Result` containing a vector of `Stmt` objects if successful, or a `ParserError` if a syntax error is encountered.
    fn body(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut body = Vec::new();

        while !self.check(TokenType::Dedent) && !self.is_at_end() {
            let stmt = self.declaration()?;
            body.push(stmt);
        }
        self.consume(TokenType::Dedent, "ExpectDedentAfterStmt")?;

        return Ok(body);
    }

    /// Matches the current token with the given token types.
    ///
    /// # Returns
    /// `true` if the current token matches one of the given types, `false` otherwise.
    fn match_token(&mut self, types: Vec<&TokenType>) -> bool {
        for token_type in types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    /// Checks if the current token matches the given token type.
    ///
    /// # Returns
    /// `true` if the current token matches the given type, `false` otherwise.
    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        };

        return self.peek().token_type == token_type;
    }

    /// Advances to the next token and returns the previous token.
    ///
    /// # Returns
    /// A reference to the previous token.
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1
        };

        return self.previous();
    }

    /// Returns a reference to the previous token.
    ///
    /// # Returns
    /// A reference to the previous token.
    fn previous(&self) -> &Token {
        return &self.tokens[self.current - 1];
    }

    /// Returns a reference to the current token.
    ///
    /// # Returns
    /// A reference to the current token.
    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }

    /// Checks if the parser has reached the end of the tokens.
    ///
    /// # Returns
    /// `true` if the parser is at the end of the tokens, `false` otherwise.
    fn is_at_end(&mut self) -> bool {
        return self.peek().token_type == TokenType::Eof;
    }

    /// Synchronizes the parser by discarding tokens until it finds a suitable point to resume parsing.
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            };

            match self.peek().token_type {
                TokenType::Def
                | TokenType::Let
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    /// Consumes the current token if it matches the given token type, otherwise returns an error.
    ///
    /// # Returns
    /// A `Result` containing the consumed token if successful, or a `ParserError` if the token does not match the expected type.
    fn consume(&mut self, token_type: TokenType, error: &str) -> Result<Token, ParserError> {
        if self.check(token_type) {
            return Ok(self.advance().clone());
        };

        return match error {
            "ExpectedVariableName" => {
                let token = self.previous().clone();
                Err(ParserError::ExpectedVariableName {
                    lexeme: token.lexeme,
                    line: token.line,
                })
            },
            "ExpectedSemicolonAfterVariableDeclaration" => {
                let token = self.previous().clone();
                Err(ParserError::ExpectedSemicolonAfterVariableDeclaration {
                    lexeme: token.lexeme,
                    line: token.line,
                })
            },
            "ExpectedLParenAfterFor" => {
                let token = self.peek();
                Err(ParserError::ExpectedLParenAfterFor {
                    line: token.line,
                })
            },
            "ExpectedSemiColonAfterForCondition" => {
                let token = self.peek();
                Err(ParserError::ExpectedSemiColonAfterForCondition {
                    line: token.line,
                })
            },
            "ExpectedRParenAfterForClauses" => {
                let token = self.peek();
                Err(ParserError::ExpectedRParenAfterForClauses {
                    line: token.line,
                })
            },
            "ExpectedLParenAfterIf" => {
                let token = self.peek();
                Err(ParserError::ExpectedLParenAfterIf {
                    line: token.line,
                })
            },
            "ExpectedLParenAfterCondition" => {
                let token = self.peek();
                Err(ParserError::ExpectedLParenAfterCondition {
                    line: token.line,
                })
            },
            "ExpectedSemicolonAfterPrint" => {
                let token = self.previous();
                Err(ParserError::ExpectedSemicolonAfterPrint {
                    value: token.lexeme.clone(),
                    line: token.line,
                })
            },
            "ExpectedSemicolonAfterReturnValue" => {
                let token = self.previous();
                Err(ParserError::ExpectedSemicolonAfterReturnValue {
                    value: token.lexeme.clone(),
                    line: token.line,
                })
            },
            "ExpectedLParenAfterWhile" => {
                let token = self.peek();
                Err(ParserError::ExpectedLParenAfterWhile {
                    line: token.line,
                })
            },
            "ExpectedRBraceAfterBlock" => {
                let token = self.peek();
                Err(ParserError::ExpectedRBraceAfterBlock {
                    line: token.line,
                })
            },
            "ExpectedRParenAfterArguments" => {
                let token = self.peek();
                Err(ParserError::ExpectedRParenAfterArguments {
                    line: token.line,
                })
            },
            "ExpectedRParenAfterExpression" => {
                let token = self.peek();
                Err(ParserError::ExpectedRParenAfterExpression {
                    line: token.line,
                })
            },
            "ExpectedExpression" => {
                let prev = self.previous();
                let token = self.peek();
                Err(ParserError::ExpectedExpression {
                    prev: prev.lexeme.clone(),
                    line: token.line,
                })
            },
            "ExpectedFunctionName" => {
                let token = self.peek();
                Err(ParserError::ExpectedFunctionName {
                    line: token.line,
                })
            },
            "ExpectedLParenAfterFunctionName" => {
                let token = self.peek();
                Err(ParserError::ExpectedLParenAfterFunctionName {
                    line: token.line,
                })
            },
            "ExpectedParameterName" => {
                let token = self.peek();
                Err(ParserError::ExpectedParameterName {
                    line: token.line,
                })
            },
            "ExpectedRBrackAfterValues" => {
                let token = self.peek();
                Err(ParserError::ExpectedRBrackAfterValues {
                    line: token.line,
                })
            },
            "ExpectedInitialiser" => {
                let token = self.peek();
                Err(ParserError::ExpectedInitializer {
                    line: token.line,
                })
            },
            "ExpectedInAfterIdentifier" => {
                let token = self.peek();
                Err(ParserError::ExpectedInAfterIdentifier {
                    line: token.line
                })
            },
            "ExpectedDotDot" => {
                let token = self.peek();
                Err(ParserError::ExpectedDotDot {
                    line: token.line
                })
            },
            "ExpectedColon" => {
                let token = self.peek();
                Err(ParserError::ExpectedColon {
                    line: token.line
                })
            },
            "ExpectedForBody" => {
                let token = self.peek();
                Err(ParserError::ExpectedBody {
                    type_: "for".to_string(),
                    line: token.line
                })
            },
            "ExpectedIfBody" => {
                let token = self.peek();
                Err(ParserError::ExpectedBody {
                    type_: "if".to_string(),
                    line: token.line
                })
            },
            "ExpectedWhileBody" => {
                let token = self.peek();
                Err(ParserError::ExpectedBody {
                    type_: "while".to_string(),
                    line: token.line
                })
            },
            "ExpectedDedent" => {
                let token = self.peek();
                Err(ParserError::ExpectedDedent {
                    line: token.line
                })
            },
            "ExpectedLParenBeforePrintValue" => {
                let token = self.peek();
                Err(ParserError::ExpectedLParenBeforePrintValue {
                    line: token.line
                })
            },
            "ExpectedRParenAfterPrintValue" => {
                let token = self.peek();
                Err(ParserError::ExpectedRParenAfterPrintValue {
                    line: token.line
                })
            },
            "ExpectedColonAfterWhileCondition" => {
                let token = self.peek();
                Err(ParserError::ExpectedColonAfterWhileCondition {
                    line: token.line
                })
            },
            _ => Err(ParserError::Unknown),
        }
    }
}
