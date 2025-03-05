use std::{cmp::min, fmt};

use crate::{error::EvaluatorError, value::{LiteralType, Value}};

const THRESHOLD: f32 = 32.0;

/// The `List` struct represents a list of values and provides methods for manipulating the list.
///
/// ## Fields
/// - `values`: A vector that stores the values in the list.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct List {
    pub values: Vec<Value>
}

impl List {
    /// Creates a new `List` instance with the given values.
    pub fn new(values: Vec<Value>) -> Self {
        return Self { values };
    }

    /// Adds a value to the end of the list.
    pub fn push(&mut self, args: Vec<Value>) -> Result<&mut List, EvaluatorError>  {
        if args.len() != 1 {
            return Err(EvaluatorError::ArgsDifferFromArity { args: args.len(), arity: 1 });
        }
        self.values.push(args[0].clone());
        return Ok(self);
    }

    /// Removes and returns the last value from the list.
    pub fn pop(&mut self) -> (Option<Value>, &mut List) {
        return (self.values.pop(), self);
    }

    /// Removes and returns the value at the specified index.
    pub fn remove(&mut self, args: Vec<Value>) -> Result<(Value, &mut List), EvaluatorError> {
        if args.len() != 1 {
            return Err(EvaluatorError::ArgsDifferFromArity { args: args.len(), arity: 1 });
        }

        if let Value::Literal(LiteralType::Num(num)) = args[0] {
            return Ok((self.values.remove(num as usize), self));
        }

        return Err(EvaluatorError::ExpectedIndexToBeANum);
    }

    /// Inserts a value at the specified index.
    pub fn insert_at(&mut self, args: Vec<Value>) -> Result<&mut List, EvaluatorError> {
        if args.len() != 2 {
            return Err(EvaluatorError::ArgsDifferFromArity { args: args.len(), arity: 2 });
        }

        if let Value::Literal(LiteralType::Num(num)) = args[0] {
            self.values.insert(num as usize, args[1].clone());
            return Ok(self);
        }
        
        return Err(EvaluatorError::ExpectedIndexToBeANum);
    }

    /// Returns the index of the specified value in the list.
    pub fn index(&self, args: Vec<Value>) -> Result<usize, EvaluatorError> {
        if args.len() != 1 {
            return Err(EvaluatorError::ArgsDifferFromArity { args: args.len(), arity: 1 });
        }

        return match self.values.iter().position(|x| x == &args[0]) {
            Some(index) => Ok(index),
            None => Err(EvaluatorError::ItemNotFound),
        }
    }

    /// Returns the length of the list.
    pub fn len(&self) -> usize {
        return self.values.len();
    }

    /// Sorts the list using the TimSort algorithm.
    pub fn tim_sort(&mut self) -> Result<&mut List, EvaluatorError> {
        let n = self.values.len();

        let mut run_length = self.calc_min_run(n as f32);

        for start in (0..n).step_by(run_length) {
            let end = min(start + run_length - 1, n - 1);
            self.insertion_sort(start, end);
        }

        if n <= 32 {
            return Ok(self);
        }

        while run_length < n {
            for left in (0..n).step_by(2 * run_length) {
                let mid = min(n - 1, left + run_length - 1);
                let right = min(n - 1, left + 2 * run_length - 1);

                if mid < right {
                    self.merge_sort(left, mid, right)?;
                }
            }
            run_length *= 2;
        }
        return Ok(self);
    }

    fn calc_min_run(&self, len: f32) -> usize {
        let mut run_len = len;
        let mut remainder: f32 = 0.0;
        while run_len > THRESHOLD {
            if run_len % 2.0 == 1.0 {
                remainder = 1.0;
            }
            run_len = run_len.floor() / 2.0;
        }
        
        return (run_len + remainder) as usize;
    }

    fn insertion_sort(&mut self, left: usize, right: usize) {
        let mut j;
        for i in left + 1..right + 1 {
            j = i;
            while j > left && self.values[j] < self.values[j - 1] {
                self.values.swap(j, j - 1);
                j -= 1;
            }
        }
    }

    fn merge_sort(&mut self, l: usize, m: usize, r: usize) -> Result<(), EvaluatorError> {
        let left_len = m - l + 1;
        let right_len = r - m;

        let left = self.values[l..=m].to_vec();
        let right = self.values[m+1..=r].to_vec();


        let mut i = 0;
        let mut j = 0;
        let mut k = l;

        while i < left_len && j < right_len {
            match (&left[i], &right[j]) {
                (Value::Literal(a), Value::Literal(b)) => {
                    match (a, b) {
                        (LiteralType::Num(n1), LiteralType::Num(n2)) => {
                            if n1 <= n2 {
                                self.values[k] = left[i].clone();
                                i += 1;
                            } else {
                                self.values[k] = right[j].clone();
                                j += 1;
                            }
                            k += 1;
                        },
                        (LiteralType::Str(s1), LiteralType::Str(s2)) => {
                            if s1 <= s2 {
                                self.values[k] = left[i].clone();
                                i += 1;
                            } else {
                                self.values[k] = right[j].clone();
                                j += 1;
                            }
                            k += 1;
                        },
                        _ => return Err(EvaluatorError::CannotCompareValues),
                    }
                },
                _ => return Err(EvaluatorError::CannotCompareValues),
            }
        }

        while i < left_len {
            self.values[k] = left[i].clone();
            i += 1;
            k += 1;
        }

        while j < right_len {
            self.values[k] = right[j].clone();
            j += 1;
            k += 1;
        }

        return Ok(());
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, value) in self.values.iter().enumerate() {
            if i > 0 {
            write!(f, ", ")?;
            }
            if let Value::Literal(LiteralType::Str(_)) = value {
                write!(f, "\"{}\"", value)?;
                continue;
            }
            write!(f, "{}", value)?;
        }
        write!(f, "]")
    }
}