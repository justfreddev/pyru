//! The `error` module defines the various error types used throughout the interpreter. These
//! errors are categorized into different enums based on the phase of execution where they occur,
//! such as lexical analysis, parsing, semantic analysis, and evaluation.
//!
//! ## Overview
//!
//! The module includes the following error enums:
//! - `LexerError`: Errors that occur during the lexical analysis phase.
//! - `ParserError`: Errors that occur during the parsing phase.
//! - `SemanticAnalyserError`: Errors that occur during the semantic analysis phase.
//! - `EvaluatorError`: Errors that occur during the evaluation phase.
//!
//! Each error variant provides detailed information about the nature of the error, including
//! the line number, position, and additional context where applicable. This helps in debugging
//! and reporting errors to the user in a meaningful way.
//!
//! ## Example
//!
//! ```rust
//! use crate::error::LexerError;
//!
//! let error = LexerError::UnexpectedCharacter {
//!     c: 'x',
//!     line: 1,
//!     start: 0,
//!     end: 1,
//! };
//!
//! println!("{}", error);
//! ```
//!
//! ## Usage
//!
//! These error types are used throughout the interpreter to handle and propagate errors in a
//! structured and consistent manner. They implement the `thiserror::Error` trait, allowing
//! them to be easily formatted and displayed.

use thiserror::Error;

use crate::{expr::Expr, stmt::Stmt};

/// Represents errors that occur during the lexical analysis phase.
#[derive(Error, Debug)]
pub enum LexerError {
    /// Occurs when a string is not properly terminated.
    #[error("Unterminated string on line {line}")]
    UnterminatedString { line: usize, start: usize, end: usize },

    /// Occurs when an unrecognized character is encountered.
    #[error("Unexpected character '{c}' on line {line}")]
    UnexpectedCharacter { c: char, line: usize, start: usize, end: usize },

    /// Occurs when the lexer expects another character but there are no more.
    #[error("No more characters left on line {line}")]
    NoCharactersLeft { line: usize, start: usize, end: usize },

    /// Occurs when the lexer reaches the end of the source but still expects another character.
    #[error("Cannot peek when at the end of the source string on line {line}")]
    CannotPeekAtTheEnd { line: usize, start: usize, end: usize },

    /// Occurs when incorrect indentation is detected.
    #[error("Incorrect indentation on line {line}")]
    IncorrectIndentation { line: usize },
}

/// Represents errors that occur during the parsing phase.
#[derive(Error, Debug)]
pub enum ParserError {
    /// Occurs when the variable name is missing from a variable declaration.
    #[error("Expected variable name after '{lexeme}' on line {line}")]
    ExpectedVariableName { lexeme: String, line: usize },

    /// Occurs when a semicolon is missing after a variable declaration.
    #[error("Expected semicolon after '{lexeme}' on line {line}")]
    ExpectedSemicolonAfterVariableDeclaration { lexeme: String, line: usize },

    /// Occurs when a left parenthesis is missing before a print value.
    #[error("Expected '(' before the print value on line {line}")]
    ExpectedLParenBeforePrintValue { line: usize },

    /// Occurs when a right parenthesis is missing after a print value.
    #[error("Expected ')' after the print value on line {line}")]
    ExpectedRParenAfterPrintValue { line: usize },

    /// Occurs when a semicolon is missing after a print statement.
    #[error("Expect ';' after print value '{value}' on line {line}")]
    ExpectedSemicolonAfterPrint { value: String, line: usize },

    /// Occurs when a semicolon is missing after a return value.
    #[error("Expect ';' after return value '{value}' on line {line}")]
    ExpectedSemicolonAfterReturnValue { value: String, line: usize },

    /// Occurs when a left parenthesis is missing after a `while` keyword.
    #[error("Expect '(' after 'while' on line {line}")]
    ExpectedLParenAfterWhile { line: usize },

    /// Occurs when a right brace is missing to close a block.
    #[error("Expect '}}' to close block on line {line}")]
    ExpectedRBraceAfterBlock { line: usize },

    /// Occurs when an alteration expression is expected but not found.
    #[error("Expected an alteration expression on line {line}")]
    ExpectedAlterationExpression { line: usize },

    /// Occurs when an invalid alteration target is encountered.
    #[error("Invalid alteration target '{target}' on line {line}")]
    InvalidAlterationTarget { target: String, line: usize },

    /// Occurs when an invalid assignment target is encountered.
    #[error("Invalid assignment target '{target}' on line {line}")]
    InvalidAssignmentTarget { target: String, line: usize },

    /// Occurs when more than 255 arguments are passed to a function.
    #[error("More than 255 arguments have been passed to {callee}")]
    TooManyArguments { callee: Expr },

    /// Occurs when a right parenthesis is missing after function arguments.
    #[error("Expect ')' after arguments on line {line}")]
    ExpectedRParenAfterArguments { line: usize },

    /// Occurs when a literal cannot be parsed into a float.
    #[error("Unable to parse literal '{value}' to a float on line {line}")]
    UnableToParseLiteralToFloat { value: String, line: usize },

    /// Occurs when a string or number is expected but not found.
    #[error("Expected a string/number, got '{value}' on line {line}")]
    ExpectedStringOrNumber { value: String, line: usize },

    /// Occurs when a right parenthesis is missing after an expression.
    #[error("Expect ')' after expression on line {line}")]
    ExpectedRParenAfterExpression { line: usize },

    /// Occurs when an expression is expected but not found.
    #[error("Expect expression after '{prev}' on line {line} (commonly due to misspelling keywords)")]
    ExpectedExpression { prev: String, line: usize },

    /// Occurs when a function name is expected but not found.
    #[error("Expect function name on line {line}")]
    ExpectedFunctionName { line: usize },

    /// Occurs when a left parenthesis is missing after a function name.
    #[error("Expect '(' after function name on line {line}")]
    ExpectedLParenAfterFunctionName { line: usize },

    /// Occurs when more than 255 parameters are passed to a function.
    #[error("More than 255 parameters have been passed to the '{name}' on line {line}")]
    TooManyParameters { name: String, line: usize },

    /// Occurs when a parameter name is expected but not found.
    #[error("Expect a parameter name on line {line}")]
    ExpectedParameterName { line: usize },

    /// Occurs when a right bracket is missing after list values.
    #[error("Expected ']' after the values of a list on line {line}")]
    ExpectedRBrackAfterValues { line: usize },

    /// Occurs when a method is called on a non-identifier.
    #[error("Can only call methods on identifiers, not '{value}' on line {line}")]
    CanOnlyCallIdentifiers { value: String, line: usize },

    /// Occurs when an initializer is missing in a `for` loop.
    #[error("Expected an initializer in the for loop on line {line}")]
    ExpectedInitializer { line: usize },

    /// Occurs when the `in` keyword is missing in a `for` loop.
    #[error("Expected the 'in' keyword on line {line}")]
    ExpectedInAfterIdentifier { line: usize },

    /// Occurs when the `..` operator is missing between ranges.
    #[error("Expected '..' between the two ranges")]
    ExpectedDotDot { line: usize },

    /// Occurs when a colon is expected but not found.
    #[error("Expected ':' at the end of line {line}")]
    ExpectedColon { line: usize },

    /// Occurs when a loop body is missing.
    #[error("Expected a body in the {type_} loop on line {line}")]
    ExpectedBody { type_: String, line: usize },

    /// Occurs when a dedent is expected but not found.
    #[error("Expected a dedent on line {line}")]
    ExpectedDedent { line: usize },

    /// Occurs when a colon is missing after a `while` loop condition.
    #[error("Expected ':' after the while loop condition on line {line}")]
    ExpectedColonAfterWhileCondition { line: usize },

    /// Represents an unknown parser error.
    #[error("Unknown parser error")]
    Unknown,
}

/// Represents errors that occur during the semantic analysis phase.
#[derive(Error, Debug)]
pub enum SemanticAnalyserError {
    /// Occurs when a statement does not match the expected statement.
    #[error("The statement provided ({stmt}), was different to the statement expected ({expected})")]
    DifferentStatement { stmt: Stmt, expected: String },

    /// Occurs when an expression does not match the expected expression.
    #[error("The expression provided ({expr}), was different to the expression expected ({expected})")]
    DifferentExpression { expr: Expr, expected: String },

    /// Occurs when a variable is already declared in the current scope.
    #[error("Already a variable named '{name}' in this scope")]
    VariableAlreadyAssignedInScope { name: String },

    /// Occurs when a variable is not found in the current or enclosing scopes.
    #[error("Couldn't find variable '{name}'")]
    VariableNotFound { name: String },

    /// Occurs when a `return` statement is used outside of a function.
    #[error("Cannot return outside of a function")]
    CannotReturnOutsideFunction,
}

/// Represents errors that occur during the evaluation phase.
#[derive(Error, Debug)]
pub enum EvaluatorError {
    /// Occurs when a statement does not match the expected statement.
    #[error("The statement provided ({stmt}), was different to the statement expected ({expected})")]
    DifferentStatement { stmt: Stmt, expected: String },

    /// Occurs when an expression does not match the expected expression.
    #[error("The expression provided ({expr}), was different to the expected ({expected})")]
    DifferentExpression { expr: Expr, expected: String },

    /// Occurs when a literal value is expected but not found.
    #[error("Expected a literal value")]
    ExpectedLiteralValue,

    /// Occurs when a list is expected in a membership expression but not found.
    #[error("Expected a list in the membership expression")]
    ExpectedList,

    /// Occurs when a number cannot be negated.
    #[error("Unable to negate number")]
    UnableToNegate,

    /// Occurs when a minus sign is expected but not found.
    #[error("Expected a minus")]
    ExpectedMinus,

    /// Occurs when a number is expected but not found.
    #[error("Expected a number")]
    ExpectedNumber,

    /// Occurs when a valid binary operator is expected but not found.
    #[error("Expected a valid binary operator")]
    ExpectedValidBinaryOperator,

    /// Occurs when a variable is undefined in the current or enclosing scopes.
    #[error("Undefined variable {name} on line {line}")]
    UndefinedVariable {
        name: String,
        start: usize,
        end: usize,
        line: usize,
    },

    /// Occurs when an alteration token is expected but not found.
    #[error("Expected an alteration token")]
    ExpectedAlterationToken,

    /// Occurs when a function or class is expected but a literal value is found.
    #[error("Expected to call a function, not a literal value")]
    ExpectedFunctionOrClass,

    /// Occurs when the number of arguments does not match the function's arity.
    #[error("Expected {arity} arguments but got {args}")]
    ArgsDifferFromArity { args: usize, arity: usize },

    /// Occurs when a function declaration is expected but not found.
    #[error("Expected the function declaration to be function statement")]
    ExpectedDeclarationToBeAFunction,

    /// Occurs when a literal value is expected for printing but not found.
    #[error("Expected to print out a literal value")]
    ExpectedToPrintLiteralValue,

    /// Occurs when a function declaration is expected but not found.
    #[error("Expected function declaration to be a function statement")]
    ExpectedFunctionStatementForDeclaration,

    /// Occurs when a list index is expected to be a number but is not.
    #[error("Expected the index to be a number value")]
    ExpectedIndexToBeANum,

    /// Occurs when a list index is out of range.
    #[error("The list index was out of range")]
    IndexOutOfRange,

    /// Occurs when a value that cannot be indexed is used as a list.
    #[error("The value cannot be indexed")]
    ValueWasNotAList,

    /// Occurs when an invalid method is called on a list.
    #[error("That method does not exist on a list")]
    InvalidListMethod,

    /// Occurs when an item cannot be found in a list.
    #[error("The item could not be found in the list")]
    ItemNotFound,

    /// Occurs when two values cannot be compared.
    #[error("The two values could not be compared")]
    CannotCompareValues,

    /// Occurs when a value passed to the hash function is not a string.
    #[error("The value passed in to the hash function must be a string")]
    CannotHashValue,
}
