# Lab 1 — Rust basics

---

## Task 0 — Toolchain and Cargo

1. Install Rust with `rustup` (rustup.rs) and check `rustc --version` and `cargo --version`.
2. Create a project and run it:
   ```bash
   cargo new lab01
   cd lab01
   cargo run
   ```
3. Look at `Cargo.toml` and `src/main.rs`. What did `cargo run` create in `target/`?
4. Try each of these and note what it does: `cargo check`, `cargo build`, `cargo build --release`, `cargo fmt`, `cargo clippy`, `cargo test`.
5. Install **rust-analyzer** in your editor. Hover over a variable to see its inferred type.

> Use `cargo check` all the time — it is much faster than `build` and gives the same errors.

---

## Task 1 — Formatting output

Print the following using a single `println!` for each line:

```
Name: Ada, age: 36
Pi to 3 places: 3.142
|   right|left    | center |
Debug: (1, "two", 3.0)
0x ff, 0b 1010
```

Hints: `{}`, `{:?}`, `{:.3}`, `{:>8}`, `{:<8}`, `{:^8}`, `{:x}`, `{:b}`. Since Rust 2021 you can write variable names inline: `println!("{name}")`.

---

## Task 2 — Variables, mutability and shadowing

1. **Predict, then run.** What does this print? Why does it compile even though `x` is not `mut`?
   ```rust
   let x = 5;
   let x = x + 1;
   {
       let x = x * 2;
       println!("inner: {x}");
   }
   println!("outer: {x}");
   ```
2. **Fix the error** without using shadowing:
   ```rust
   let count = 0;
   count += 1;
   println!("{count}");
   ```
3. Shadowing lets you change a variable's _type_. Read a string `"  42  "`, trim it, and parse it into a `u32` — reusing the name `input` for every step.
4. What is the difference between `let mut x` and shadowing with `let x` again? When would you pick each one? Write two or three sentences as a comment.
5. Declare a constant `MAX_POINTS: u32 = 100_000` at the top of the file. How is `const` different from an immutable `let`?

---

## Task 3 — Scalar types and integer overflow

1. Print `i8::MIN`, `i8::MAX`, `u8::MAX`, `i32::MAX`, `u64::MAX`, `f64::EPSILON`.
2. What happens with `let x: u8 = 255; let y = x + 1;` in a **debug** build? In a **release** build? (Try `cargo run --release`.) Why do you think Rust behaves differently in the two?
3. Rust makes overflow handling explicit. For `250u8 + 10`, print the result of `checked_add`, `wrapping_add`, `saturating_add` and `overflowing_add`.
4. Write
   ```rust
   fn factorial(n: u32) -> Option<u64>
   ```
   which returns `None` when the result does not fit in a `u64`. Use `checked_mul`.
   `factorial(5) == Some(120)`, `factorial(20) == Some(2432902008176640000)`, `factorial(21) == None`.
   _(`Option` is covered properly in Lab 3. For now, `Some(v)` means "there is a value" and `None` means "there is none".)_
5. Casting: what do `300_i32 as u8`, `-1_i32 as u32`, `3.99_f64 as i32` and `-3.99_f64 as u32` give? Why is `as` considered dangerous, and what could you use instead? (Look up `u8::try_from`.)

---

## Task 4 — Functions and expressions

In Rust almost everything is an **expression**. A block's value is its last expression, written _without_ a semicolon.

1. Write `fn celsius_to_fahrenheit(c: f64) -> f64` and its inverse **without the `return` keyword**.
2. Write `fn sign(x: i32) -> String` that returns `"negative"`, `"zero"` or `"positive"`. Use `if` as an expression.
3. Why does this not compile? Fix it by changing **one character**.
   ```rust
   fn square(x: i32) -> i32 {
       x * x;
   }
   ```
4. Use a block expression to set a variable:
   ```rust
   let y = {
       let a = 3;
       a * a + 1
   };
   ```
   What is `y`? What would `y` be if the last line ended with `;`?

---

## Task 5 — Control flow

Write these functions. Each one practises a different construct.

| Function                          | Construct to use                               | Examples                                                  |
| --------------------------------- | ---------------------------------------------- | --------------------------------------------------------- |
| `fn fizzbuzz(n: u32) -> String`   | `match` on the tuple `(n % 3, n % 5)`          | `3 → "Fizz"`, `10 → "Buzz"`, `15 → "FizzBuzz"`, `7 → "7"` |
| `fn collatz_steps(n: u64) -> u32` | `while`                                        | `1 → 0`, `6 → 8`, `27 → 111`                              |
| `fn gcd(a: u64, b: u64) -> u64`   | `loop` or `while`, Euclid's algorithm          | `gcd(48, 18) == 6`                                        |
| `fn is_prime(n: u64) -> bool`     | `for` over a range, early `return`             | `is_prime(2)`, `!is_prime(1)`, `is_prime(97)`             |
| `fn fib(n: u32) -> u64`           | `for` + tuple assignment `(a, b) = (b, a + b)` | `fib(0) == 0`, `fib(10) == 55`, `fib(50) == 12586269025`  |
| `fn grade(points: u32) -> char`   | `match` with ranges `90..=100`                 | `95 → 'A'`, `72 → 'C'`, `30 → 'F'`                        |

Then print FizzBuzz for 1..=20 using your function and a `for` loop.

**Questions**

- Remove one arm from the `match` in `grade`. What does the compiler say? Why is **exhaustiveness** useful?
- What is the difference between `1..10` and `1..=10`?
- Try `for i in (1..=5).rev()` and `for i in (0..20).step_by(5)`.

---

## Task 6 — `loop` with a value, and labelled loops

1. `loop` can return a value through `break value`. Write `fn first_power_of_two_above(n: u64) -> u64` using `loop` and `break`.
   `first_power_of_two_above(5) == 8`, `first_power_of_two_above(8) == 16`.
2. Print this triangle with two nested `for` loops. Stop each row as soon as the product is greater than `limit` (here `limit = 12`), using `continue 'outer`:
   ```
   1 2 3 4 5 6 7 8 9
   2 4 6 8 10 12
   3 6 9 12
   4 8 12
   5 10
   6 12
   7
   8
   9
   ```

---

## Task 7 — Tuples and arrays

1. Write `fn min_max_avg(values: Vec<i32>) -> (i32, i32, f64)`. Call it with `min_max_avg(vec![3, -1, 7, 2])`, which should return `(-1, 7, 2.75)`. Destructure the result: `let (lo, hi, avg) = ...;`.
2. Create an array `let grid = [[0u8; 3]; 3];`, make it mutable, set the diagonal to `1` (a plain indexed loop `for i in 0..3 { grid[i][i] = 1; }` works) and print it with `{:?}`.
3. What happens if you index `grid[5][0]`? Do you get a compile error or a runtime panic? Try a constant index, then an index computed at runtime.
4. Write `fn swap_pair(p: (i32, String)) -> (String, i32)`.
