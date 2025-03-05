use thiserror::Error;

use crate::{ expr::Expr, stmt::Stmt };

#[derive(Error, Debug)]
pub enum LexerError {
    // Occurs when '"' is found, but there is not another one to close the string
    #[error("Unterminated string on line {line}")]
    UnterminatedString { line: usize, start: usize, end: usize },

    // Occurs when an unrecognised character is found
    #[error("Unexpected character '{c}' on line {line}")]
    UnexpectedCharacter { c: char, line: usize, start: usize, end: usize },

    // Occurs when the lexer expects another character but there are no more
    #[error("No more characters left on line {line}")]
    NoCharactersLeft { line: usize, start: usize, end: usize },

    // Occurs when the lexer reaches the end of the source but still expects another character
    #[error("Cannot peek when at the end of the source string on line {line}")]
    CannotPeekAtTheEnd { line: usize, start: usize, end: usize },

    #[error("Incorrect indentation on line {line}")]
    IncorrectIndentation { line: usize }
}

#[derive(Error, Debug)]
pub enum ParserError {
    // Occurs when the variable name is missing from a variable declaration
    #[error("Expected variable name after '{lexeme}' on line {line}")]
    ExpectedVariableName {
        lexeme: String,
        line: usize,
    },

    #[error("Expected semicolon after '{lexeme}' on line {line}")]
    ExpectedSemicolonAfterVariableDeclaration {
        lexeme: String,
        line: usize,
    },

    #[error("Expected '(' before the print value on line {line}")]
    ExpectedLParenBeforePrintValue {
        line: usize,
    },

    #[error("Expected ')' after the print value on line {line}")]
    ExpectedRParenAfterPrintValue {
        line: usize,
    },

    #[error("Expect ';' after print value '{value}' on line {line}")]
    ExpectedSemicolonAfterPrint {
        value: String,
        line: usize,
    },

    #[error("Expect ';' after return value '{value}' on line {line}")]
    ExpectedSemicolonAfterReturnValue {
        value: String,
        line: usize,
    },

    #[error("Expect '(' after 'while' on line {line}")]
    ExpectedLParenAfterWhile {
        line: usize,
    },

    #[error("Expect '}}' to close block on line {line}")]
    ExpectedRBraceAfterBlock {
        line: usize,
    },

    #[error("Expected an alteration expression on line {line}")]
    ExpectedAlterationExpression {
        line: usize,
    },

    #[error("Invalid alteration target '{target}' on line {line}")]
    InvalidAlterationTarget {
        target: String,
        line: usize,
    },

    #[error("Invalid assignment target '{target}' on line {line}")]
    InvalidAssignmentTarget {
        target: String,
        line: usize,
    },

    #[error("More than 255 arguments have been passed to {callee}")]
    TooManyArguments { callee: Expr },

    #[error("Expect ')' after arguments on line {line}")]
    ExpectedRParenAfterArguments {
        line: usize,
    },

    #[error("Unable to parse literal '{value}' to a float on line {line}")]
    UnableToParseLiteralToFloat {
        value: String,
        line: usize,
    },

    #[error("Expected a string/number, got '{value}' on line {line}")]
    ExpectedStringOrNumber {
        value: String,
        line: usize,
    },

    #[error("Expect ')' after expression on line {line}")]
    ExpectedRParenAfterExpression {
        line: usize,
    },

    #[error("Expect expression after '{prev}' on line {line} (commonly due to mispelling keywords)")]
    ExpectedExpression {
        prev: String,
        line: usize,
    },

    #[error("Expect function name on line {line}")]
    ExpectedFunctionName {
        line: usize,
    },

    #[error("Expect '(' after function name on line {line}")]
    ExpectedLParenAfterFunctionName {
        line: usize,
    },

    #[error("More than 255 parameters have been passed to the '{name}' on line {line}")]
    TooManyParameters {
        name: String,
        line: usize,
    },

    #[error("Expect a parameter name on line {line}")]
    ExpectedParameterName {
        line: usize,
    },

    #[error("Expected ']' after the values of a list on line {line}")]
    ExpectedRBrackAfterValues {
        line: usize,
    },

    #[error("Can only call methods on identifiers, not '{value}' on line {line}")]
    CanOnlyCallIdentifiers {
        value: String,
        line: usize,
    },

    #[error("Expected an initializer in the for loop on line {line}")]
    ExpectedInitializer {
        line: usize
    },

    #[error("Expected the 'in' keyword on line {line}")]
    ExpectedInAfterIdentifier {
        line: usize
    },

    #[error("Expected '..' between the two ranges")]
    ExpectedDotDot {
        line: usize
    },

    #[error("Expected at the end of line {line}")]
    ExpectedColon {
        line: usize
    },

    #[error("Expected a body in the {type_} loop on line {line}")]
    ExpectedBody {
        type_: String,
        line: usize
    },

    #[error("Expected a dedent on line {line}")]
    ExpectedDedent {
        line: usize
    },

    #[error("Expected ':' after the while loop condition on line {line}")]
    ExpectedColonAfterWhileCondition {
        line: usize
    }, 

    #[error("Unknown parser error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum SemanticAnalyserError {
    #[error("The statement provided ({stmt}), was different to the statement expected ({expected})")]
    DifferentStatement { stmt: Stmt, expected: String },

    #[error("The expression provided ({expr}), was different to the expression expected ({expected})")]
    DifferentExpression { expr: Expr, expected: String },

    #[error("Already a variable named '{name}' in this scope")]
    VariableAlreadyAssignedInScope { name: String },

    #[error("Couldn't find variable '{name}'")]
    VariableNotFound { name: String },

    #[error("Cannot return outside of a function")]
    CannotReturnOutsideFunction,
}

#[derive(Error, Debug)]
pub enum EvaluatorError {
    #[error("The statement provided ({stmt}), was different to the statement expected ({expected})")]
    DifferentStatement { stmt: Stmt, expected: String },

    #[error("The expression provided ({expr}), was different to the expected ({expected})")]
    DifferentExpression { expr: Expr, expected: String },

    #[error("Expected a literal value")]
    ExpectedLiteralValue,

    #[error("Expected a list in the membership expression")]
    ExpectedList,

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
        line: usize
    },

    #[error("Expected an alteration token")]
    ExpectedAlterationToken,

    #[error("Expected to call a function, not a literal value")]
    ExpectedFunctionOrClass,

    #[error("Expected {arity} arguments but got {args}")]
    ArgsDifferFromArity { args: usize, arity: usize },

    #[error("Expected the function declaration to be function statement")]
    ExpectedDeclarationToBeAFunction,

    #[error("Expected to print out a literal value")]
    ExpectedToPrintLiteralValue,

    #[error("Expected function declaration to be a function statement")]
    ExpectedFunctionStatementForDeclaration,

    #[error("Expected the index to be a number value")]
    ExpectedIndexToBeANum,

    #[error("The list index was out of range")]
    IndexOutOfRange,

    #[error("The value cannot be indexed")]
    ValueWasNotAList,

    #[error("That method does not exist on a list")]
    InvalidListMethod,

    #[error("The item could not be found in the list")]
    ItemNotFound,

    #[error("The two values could not be compared")]
    CannotCompareValues,

    #[error("The value passed in to the hash function must be a string")]
    CannotHashValue,
}
