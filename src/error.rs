use thiserror::Error;

use crate::{expr::Expr, stmt::Stmt, token::TokenType};

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unterminated string on line {line}")]
    UnterminatedString { line: usize },
    #[error("Unexpected character '{c}' on line {line}")]
    UnexpectedCharacter { c: char, line: usize },
    #[error("No more characters left on line {line}")]
    NoCharactersLeft { line: usize },
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Expected variable name after {lexeme} of type {token_type} on line {line}")]
    ExpectedVariableName {
        token_type: TokenType,
        lexeme: String,
        line: usize,
    },

    #[error("Expected semicolon after {lexeme} of type {token_type} on line {line}")]
    ExpectedSemicolonAfterVariableDeclaration {
        token_type: TokenType,
        lexeme: String,
        line: usize,
    },

    #[error("Expect '(' after 'for' on line {line}")]
    ExpectedLParenAfterFor {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Expect ';' after for loop condition on line {line}")]
    ExpectedSemiColonAfterForCondition {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Expect ')' after for loop clauses on line {line}")]
    ExpectedRParenAfterForClauses {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Expect '(' after 'if' on line {line}")]
    ExpectedLParenAfterIf {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Expect '(' after if condition on line {line}")]
    ExpectedLParenAfterCondition {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Expect ';' after print value on line {line}")]
    ExpectedSemicolonAfterPrintValue {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Expect ';' after return value on line {line}")]
    ExpectedSemicolonAfterReturnValue {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Expect '(' after 'while' on line {line}")]
    ExpectedLParenAfterWhile {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Expect '}}' after block on line {line}")]
    ExpectedRBraceAfterBlock {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Expected an alteration expression on line {line}")]
    ExpectedAlterationExpression {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Invalid alteration target on line {line}")]
    InvalidAlterationTarget {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Invalid assignment target on line {line}")]
    InvalidAssignmentTarget {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("More than 255 arguments have been passed to {callee}")]
    TooManyArguments { callee: Expr },

    #[error("Expect ')' after arguments on line {line}")]
    ExpectedRParenAfterArguments {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Unable to parse literal to a float on line {line}")]
    UnableToParseLiteralToFloat {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Expected a string/number on line {line}")]
    ExpectedStringOrNumber {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Expect ')' after expression on line {line}")]
    ExpectedRParenAfterExpression {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Expect expression on line {line} (commonly due to mispelling keywords)")]
    ExpectedExpression {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Expect function name on line {line}")]
    ExpectedFunctionName {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Expect '(' after function name on line {line}")]
    ExpectedLParenAfterFunctionName {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("More than 255 parameters have been passed on line {line}")]
    TooManyParameters {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Expect a parameter name on line {line}")]
    ExpectedParameterName {
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Unknown parser error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum SemanticAnalyserError {
    #[error("The statement provided ({stmt}), was different to the statement expected (expected)")]
    DifferentStatement { stmt: Stmt, expected: String },

    #[error(
        "The expression provided ({expr}), was different to the expression expected (expected)"
    )]
    DifferentExpression { expr: Expr, expected: String },

    #[error("Already a variable named {name} in this scope")]
    VariableAlreadyAssignedInScope { name: String },

    #[error("Couldn't find variable {name}")]
    VariableNotFound { name: String },

    #[error("Can't return outside of a function")]
    CannotReturnOutsideFunction,
}

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("Expected a group expression")]
    ExpectedGroupExpression,

    #[error("Expected a unary expression")]
    ExpectedUnaryExpression,

    #[error("Expected a binary expression")]
    ExpectedBinaryExpression,

    #[error("Expected a variable expression")]
    ExpectedVariableExpression,

    #[error("Expected an assignment expression")]
    ExpectedAssignmentExpression,

    #[error("Expected a logical expression")]
    ExpectedLogicalExpression,

    #[error("Expected an alteration expression")]
    ExpectedAlterationExpression,

    #[error("Expected a call expression")]
    ExpectedCallExpression,

    #[error("Expected an expression statement")]
    ExpectedExpressionStatement,

    #[error("Expected a print statement")]
    ExpectedPrintStatement,

    #[error("Expected a var statement")]
    ExpectedVarStatement,

    #[error("Expected a block statement")]
    ExpectedBlockStatement,

    #[error("Expected an if statement")]
    ExpectedIfStatement,

    #[error("Expected a while statement")]
    ExpectedWhileStatement,

    #[error("Expected a for statement")]
    ExpectedForStatement,

    #[error("Expected a function statement")]
    ExpectedFunctionStatement,

    #[error("Expected a return statement")]
    ExpectedReturnStatement,

    #[error("Expected a literal value")]
    ExpectedLiteralValue,

    #[error("Unable to negate number")]
    UnableToNegate,

    #[error("Expected a minus")]
    ExpectedMinus,

    #[error("Expected a number")]
    ExpectedNumber,

    #[error("Expected a valid binary operator")]
    ExpectedValidBinaryOperator,

    #[error("Undefined variable {name} on line {line}")]
    UndefinedVariable {
        name: String,
        start: usize,
        end: usize,
        line: usize,
    },

    #[error("Expected an alteration token")]
    ExpectedAlterationToken,

    #[error("Expected to call a function/class, not a literal value")]
    ExpectedFunctionOrClass,

    #[error("Expected {arity} arguments but got {args}")]
    ArgsDifferFromArity { args: usize, arity: usize },

    #[error("Expected the function declaration to be function statement")]
    ExpectedDeclarationToBeAFunction,

    #[error("Expected to print out a literal value")]
    ExpectedToPrintLiteralValue,

    #[error("Expected function declaration to be a function statement")]
    ExpectedFunctionStatementForDeclaration,
}