use std::{cmp::min, fmt};

use crate::{error::InterpreterError, value::{LiteralType, Value}};

const THRESHOLD: f32 = 32.0;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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

    // https://www.geeksforgeeks.org/timsort/
    // https://www.baeldung.com/cs/timsort
    pub fn tim_sort(&mut self) -> Result<List, InterpreterError> {
        let mut list = self.values.clone();
        let n = list.len();

        let run_length = self.calc_min_run(n);

        for start in (0..n).step_by(run_length) {
            let end = min(start + run_length - 1, n - 1);
            self.insertion_sort(&mut list, start, end);
        }

        if n <= 32 {
            return Ok(List::new(list));
        }

        let mut size = run_length;

        while size < n {
            for left in (0..n).step_by(2 * size) {
                let mid = min(n - 1, left + size - 1);
                let right = min(n - 1, left + 2 * size - 1);

                if mid < right {
                    self.merge_sort(&mut list, left, mid, right)?;
                }
            }
            size *= 2;
        }
        
        return Ok(List::new(list));
    }

    fn calc_min_run(&self, len: usize) -> usize {
        let mut run_len = len as f32;
        let mut remainder: f32 = 0.0;
        while run_len > THRESHOLD {
            if run_len % 2.0 == 1.0 {
                remainder = 1.0;
            }
            run_len = run_len.floor() / 2.0;
        }
        
        return (run_len + remainder) as usize;
    }

    fn insertion_sort(&self, list: &mut Vec<Value>, left: usize, right: usize) {
        let mut j;
        for i in left + 1..right + 1 {
            j = i;
            while j > left && list[j] < list[j - 1] {
                list.swap(j, j - 1);
                j -= 1;
            }
        }
    }

    fn merge_sort(&self, list: &mut Vec<Value>, l: usize, m: usize, r: usize) -> Result<(), InterpreterError> {
        let left_len = m - l + 1;
        let right_len = r - m;

        let left = list[l..=m].to_vec();
        let right = list[m+1..=r].to_vec();


        let mut i = 0;
        let mut j = 0;
        let mut k = l;

        while i < left_len && j < right_len {
            match (&left[i], &right[j]) {
                (Value::Literal(a), Value::Literal(b)) => {
                    match (a, b) {
                        (LiteralType::Num(n1), LiteralType::Num(n2)) => {
                            if n1 <= n2 {
                                list[k] = left[i].clone();
                                i += 1;
                            } else {
                                list[k] = right[j].clone();
                                j += 1;
                            }
                            k += 1;
                        },
                        (LiteralType::Str(s1), LiteralType::Str(s2)) => {
                            if s1 <= s2 {
                                list[k] = left[i].clone();
                                i += 1;
                            } else {
                                list[k] = right[j].clone();
                                j += 1;
                            }
                            k += 1;
                        },
                        _ => return Err(InterpreterError::CannotCompareValues),
                    }
                },
                _ => return Err(InterpreterError::CannotCompareValues),
            }
        }

        while i < left_len {
            list[k] = left[i].clone();
            i += 1;
            k += 1;
        }

        while j < right_len {
            list[k] = right[j].clone();
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
            write!(f, "{}", value)?;
        }
        write!(f, "]")
    }
}