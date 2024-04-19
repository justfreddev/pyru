use std::fmt;

use crate::{error::InterpreterError, value::{LiteralType, Value}};

#[derive(Clone, Debug, PartialEq)]
pub struct List {
    pub values: Vec<Value>
}

impl List {
    pub fn new(values: Vec<Value>) -> Self {
        return Self { values };
    }

    pub fn push(&mut self, args: Vec<Value>) -> Result<&mut List, InterpreterError>  {
        if args.len() != 1 {
            return Err(InterpreterError::ArgsDifferFromArity { args: args.len(), arity: 1 });
        }
        self.values.push(args[0].clone());
        return Ok(self);
    }

    pub fn pop(&mut self) -> (Option<Value>, &mut List) {
        return (self.values.pop(), self);
    }

    pub fn remove(&mut self, args: Vec<Value>) -> Result<(Value, &mut List), InterpreterError> {
        if args.len() != 1 {
            return Err(InterpreterError::ArgsDifferFromArity { args: args.len(), arity: 1 });
        }

        if let Value::Literal(LiteralType::Num(num)) = args[0] {
            return Ok((self.values.remove(num as usize), self));
        }

        return Err(InterpreterError::ExpectedIndexToBeANum);
    }

    pub fn insert_at(&mut self, args: Vec<Value>) -> Result<&mut List, InterpreterError> {
        if args.len() != 2 {
            return Err(InterpreterError::ArgsDifferFromArity { args: args.len(), arity: 2 });
        }

        if let Value::Literal(LiteralType::Num(num)) = args[0] {
            self.values.insert(num as usize, args[1].clone());
            return Ok(self);
        }
        
        return Err(InterpreterError::ExpectedIndexToBeANum);
    }

    pub fn index(&self, args: Vec<Value>) -> Result<usize, InterpreterError> {
        if args.len() != 1 {
            return Err(InterpreterError::ArgsDifferFromArity { args: args.len(), arity: 1 });
        }

        return match self.values.iter().position(|x| x == &args[0]) {
            Some(index) => Ok(index),
            None => Err(InterpreterError::ItemNotFound),
        }
    }

    pub fn len(&self) -> usize {
        return self.values.len();
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, value) in self.values.iter().enumerate() {
            if i > 0 {
            write!(f, ", ")?;
            }
            write!(f, "{}", value)?;
        }
        write!(f, "]")
    }
}