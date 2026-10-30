# Lab 4 — Result and errors

---

## Task 0 — Reverse Polish Notation

In this lab we build a calculator for **Reverse Polish Notation** (RPN). In RPN the operator comes _after_ its operands, so no parentheses are needed:

| Usual notation | RPN         |
| -------------- | ----------- |
| `3 + 4`        | `3 4 +`     |
| `(3 + 4) * 2`  | `3 4 + 2 *` |
| `3 + 4 * 2`    | `3 4 2 * +` |

Evaluation uses a stack: a number is pushed, and an operator pops two values and pushes the result.

1. Evaluate `5 1 2 + 4 * + 3 -` on paper. Write down the stack after every token.
2. Write `(1 + 2) * (3 + 4)` and `10 / (4 - 2)` in RPN.
3. What goes wrong with `1 +`, `1 2` and `4 0 /`?

---

## Task 1 — Operators and errors

`Result<T, E>` is either `Ok(value)` or `Err(error)`. Unlike `Option`, the failure case carries information about **what** went wrong.

1. Define the operators:
   ```rust
   enum Op { Add, Sub, Mul, Div }
   ```
2. Define everything that can go wrong in the calculator:
   ```rust
   #[derive(Debug, PartialEq)]
   enum CalcError {
       EmptyInput,
       StackUnderflow,         // an operator found fewer than two values
       DivisionByZero,
       UnknownToken(String),
       LeftoverValues(usize),  // values left on the stack besides the result
   }
   ```
   _Why do we derive `PartialEq`? (Hint: Task 2.)_

---

## Task 2 — Tests first

In **test-driven development** (TDD) you write a test _before_ the code it checks, and work in short cycles:

1. **red** — write a test for the next small piece of behaviour and watch it fail
2. **green** — write just enough code to make it pass
3. **refactor** — clean up the code while all tests stay green

The tests then describe what the program should do, and tell you when you are done.

1. Start with a function that compiles but does nothing yet. `todo!()` panics when it is called:
   ```rust
   fn eval_rpn(src: &str) -> Result<f64, CalcError> {
       todo!()
   }
   ```
2. Tests live next to the code, in a module that is only compiled by `cargo test`:

   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;   // everything from the parent module

       #[test]
       fn adds_and_multiplies() {
           assert_eq!(eval_rpn("3 4 + 2 *"), Ok(14.0));
       }
   }
   ```

   Write one test for **every** row of this table:

   | Input         | Result                   |
   | ------------- | ------------------------ |
   | `"3 4 + 2 *"` | `Ok(14.0)`               |
   | `"10 4 -"`    | `Ok(6.0)`                |
   | `"-2.5 2 *"`  | `Ok(-5.0)`               |
   | `"1 +"`       | `Err(StackUnderflow)`    |
   | `"4 0 /"`     | `Err(DivisionByZero)`    |
   | `"2 3 %"`     | `Err(UnknownToken("%"))` |
   | `"1 2"`       | `Err(LeftoverValues(1))` |
   | `""`, `"   "` | `Err(EmptyInput)`        |

3. Run `cargo test`. Every test should fail: this is the **red** step. Read the output.

In Tasks 3 and 4, run `cargo test` after every change and watch the tests turn green one by one.

---

## Task 3 — A stack that reports errors

```rust
struct Stack { items: Vec<f64> }
```

1. `fn new() -> Self`, `fn len(&self) -> usize` and `fn push(&mut self, x: f64)`.
2. `fn pop(&mut self) -> Result<f64, CalcError>` — `StackUnderflow` when the stack is empty. _(Hint: `self.items.pop().ok_or(...)` turns an `Option` into a `Result`.)_
3. `fn apply(&mut self, op: Op) -> Result<(), CalcError>` — pop `b`, pop `a`, push `a op b`. Use `?` on both `pop` calls. Dividing by zero (`b == 0.0`) is `DivisionByZero`.
   _Why is `b` popped first? Try `10 4 -` with the order swapped._

---

## Task 4 — Evaluating RPN

1. Implement `eval_rpn`:
   - First reject empty input. `"   "` is not `""`, but `src.trim().is_empty()` catches both.
   - Go through `src.split_whitespace()` and `match` on each token:
     - `"+"`, `"-"`, `"*"`, `"/"` — apply the operator, with `?`
     - anything else — `token.parse::<f64>()` returns a `Result`. On `Ok(x)` push `x`; on `Err(_)` return `UnknownToken` with `token.to_string()`.
   - At the end exactly one value must be left on the stack: that is the result. If there are more, return `LeftoverValues`.

   You are done when all tests from Task 2 pass.

2. **Question:** inside a function returning `Result`, what does `expr?` do, step by step?

---

## Task 5 — Error messages

`{:?}` prints `UnknownToken("%")`, which is fine for a programmer but not for a user. Give `CalcError` human-readable messages by implementing `Display`. This is your first **trait implementation**. Lab 5 explains traits; for now, follow this shape:

```rust
use std::fmt;

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcError::EmptyInput => write!(f, "empty input"),
            // ... one arm per variant
        }
    }
}
```

Messages: `"stack underflow"`, `"division by zero"`, `"unknown token '%'"`, `"1 extra value(s) left on the stack"`.

`write!` works like `format!`, but writes into `f`. Once `Display` is implemented, `{}` uses it: `println!("{e}")` prints `unknown token '%'`, while `println!("{e:?}")` still prints `UnknownToken("%")`.

---

## Task 6 — An interactive calculator

Make `main` a small interactive calculator:

```
rpn> 3 4 + 2 *
= 14
rpn> 1 +
error: stack underflow
rpn>
```

An error must **not** end the program. It ends at the end of input: Ctrl-D (Ctrl-Z then Enter on Windows).

1. Copy this helper. It shows a prompt and reads one line, like Python's `input()`, and returns `None` when there is no more input:
   ```rust
   use std::io::Write;

   fn input(prompt: &str) -> Option<String> {
       print!("{prompt}");
       std::io::stdout().flush().unwrap();  // print! does not end the line, so the prompt may not appear without flush
       std::io::stdin().lines().next()?.ok()
   }
   ```
   **Question:** `next()` returns an `Option<io::Result<String>>`. What do `?` and `.ok()` do with it?
2. In `main`, loop with `while let Some(line) = input("rpn> ") { ... }`. For each line print `= result` or `error: message`. Use `{}`, so that the messages from Task 5 appear.

**Question:** why does the loop stop when you press Ctrl-D?
