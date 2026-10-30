// 1 + 2
// PN:  + 1 2
// RPN: 1 2 +

// Task 0

// 5 1 2 + 4 * + 3 -
// 5 3 4 * + 3 -
// 5 12 + 3 -
// 17 3 -
// 14

// (1 + 2) * (3 + 4)
// 1 2 + 3 4 + *

// 10 / (4 - 2)
// 10 4 2 - /

// 1 + Stack underflow
// 1 2 Leftover values
// 4 0 / Division by zero

//! Lab 4 — Result and errors (exercises/lab04-result-errors.md)

use std::fmt;
use std::io::Write;

// Task 1 — Operators and errors

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
enum CalcError {
    EmptyInput,
    StackUnderflow, // an operator found fewer than two values
    DivisionByZero,
    UnknownToken(String),
    LeftoverValues(usize), // values left on the stack besides the result
}

// Task 3 — A stack that reports errors

struct Stack {
    items: Vec<f64>,
}

impl Stack {
    fn new() -> Self {
        Stack { items: vec![] }
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn push(&mut self, x: f64) {
        self.items.push(x);
    }

    fn pop(&mut self) -> Result<f64, CalcError> {
        self.items.pop().ok_or(CalcError::StackUnderflow)
    }

    fn apply(&mut self, op: Op) -> Result<(), CalcError> {
        let b = self.pop()?;
        let a = self.pop()?;

        let result = match op {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div if b == 0.0 => return Err(CalcError::DivisionByZero),
            Op::Div => a / b,
        };

        self.push(result);
        Ok(())
    }
}

// Task 4 — Evaluating RPN

fn eval_rpn(src: &str) -> Result<f64, CalcError> {
    if src.trim().is_empty() {
        return Err(CalcError::EmptyInput);
    }

    let mut stack = Stack::new();

    for token in src.split_whitespace() {
        match token {
            "+" => stack.apply(Op::Add)?,
            "-" => stack.apply(Op::Sub)?,
            "*" => stack.apply(Op::Mul)?,
            "/" => stack.apply(Op::Div)?,
            token => match token.parse::<f64>() {
                Ok(x) => stack.push(x),
                Err(_) => return Err(CalcError::UnknownToken(token.to_string())),
            },
        }
    }

    let leftovers = stack.len() - 1;

    if leftovers > 0 {
        return Err(CalcError::LeftoverValues(leftovers));
    }

    stack.pop()
}

// Task 5 — Error messages

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcError::EmptyInput => write!(f, "empty input"),
            CalcError::StackUnderflow => write!(f, "stack underflow"),
            CalcError::DivisionByZero => write!(f, "division by zero"),
            CalcError::UnknownToken(t) => write!(f, "unknown token '{t}'"),
            CalcError::LeftoverValues(n) => write!(f, "{n} extra value(s) left on the stack"),
        }
    }
}

// Task 6 — An interactive calculator

fn input(prompt: &str) -> Option<String> {
    print!("{prompt}");
    std::io::stdout().flush().unwrap();
    std::io::stdin().lines().next()?.ok()
}

pub fn run() {
    while let Some(line) = input("rpn> ") {
        match eval_rpn(&line) {
            Ok(v) => println!("= {v}"),
            Err(e) => println!("error: {e}"),
        }
    }
}

// Task 2 — Tests first

#[cfg(test)]
mod tests {
    use super::*; // everything from the parent module

    #[test]
    fn adds_and_multiplies() {
        assert_eq!(eval_rpn("3 4 + 2 *"), Ok(14.0));
    }

    #[test]
    fn subtracts() {
        assert_eq!(eval_rpn("10 4 -"), Ok(6.0));
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(eval_rpn("-2.5 2 *"), Ok(-5.0));
    }

    #[test]
    fn stack_underflow() {
        assert_eq!(eval_rpn("1 +"), Err(CalcError::StackUnderflow));
    }

    #[test]
    fn division_by_zero() {
        assert_eq!(eval_rpn("4 0 /"), Err(CalcError::DivisionByZero));
    }

    #[test]
    fn unknown_token() {
        assert_eq!(
            eval_rpn("2 3 %"),
            Err(CalcError::UnknownToken("%".to_string()))
        );
    }

    #[test]
    fn leftover_values() {
        assert_eq!(eval_rpn("1 2"), Err(CalcError::LeftoverValues(1)));
    }

    #[test]
    fn empty_input() {
        assert_eq!(eval_rpn(""), Err(CalcError::EmptyInput));
        assert_eq!(eval_rpn("   "), Err(CalcError::EmptyInput));
    }
}
