# Lab 3 — Enums and Option

---

## Task 1 — Simple enums

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction { North, East, South, West }
```

1. In an `impl Direction` block, write `fn turn_right(self) -> Direction` using `match`. Then write `fn opposite(self) -> Direction` and `fn turn_left(self) -> Direction` **using `turn_right`**.
2. A robot starts at `(0, 0)` facing North. Write `fn walk(commands: &str) -> (i32, i32)`: `F` moves one step forward, `L` and `R` turn, any other character is ignored. x grows to the East, y grows to the North.
   `walk("FFRFF") == (2, 2)`, `walk("LLFF?") == (0, -2)`
3. Add a fifth variant `Up` to the enum and run `cargo check`. Where does the compiler complain? Why is this **exhaustiveness check** useful? Remove `Up` again.

---

## Task 2 — Enums with data

Each variant can carry its own data:

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}
```

1. `fn area(&self) -> f64`.
2. `fn describe(&self) -> String`, using a **match guard** (`if` after a pattern):
   - a circle with radius `0.0` → `"a point"` _(a literal in the pattern, `Circle { radius: 0.0 }`, is enough here)_
   - a rectangle with `width == height` → `"a square of side 2"` (for side 2)
   - otherwise → `"a circle"`, `"a rectangle"`
3. **Question:** in Java you would write an abstract class `Shape` with subclasses. Which change is easier with the enum — adding a new _operation_ or a new _shape_? And with classes?

---

## Task 3 — `Option`

Rust has no `null`. A value that may be missing has type `Option<T>`, which is either `Some(value)` or `None`, and the compiler makes you handle both.

1. `fn find_index(values: &[i32], target: i32) -> Option<usize>` — the position of the first match. _Hint: `for (i, &v) in values.iter().enumerate()` gives you each index together with its value._
   `find_index(&[5, 6, 7], 7) == Some(2)`, `find_index(&[5, 6, 7], 9) == None`
2. `fn parse_point(s: &str) -> Option<(i32, i32)>` for inputs such as `"3,4"` or `" -1 , 7 "`. Use `split_once(',')`, `trim()` and `parse().ok()`, plus the **`?` operator**: inside a function returning `Option`, `x?` means "if `x` is `None`, return `None` right now". The body should be two or three lines.
   `parse_point("3;4") == None`, `parse_point("3,x") == None`
3. Print `x = 3, y = 4` or `invalid` for a parsed point — **twice**, using:
   - `if let Some((x, y)) = ... { } else { }`
   - `let Some((x, y)) = ... else { println!("invalid"); return; };`
4. Write each function as **one expression**, without `match`:
   - `fn double_first(v: &[i32]) -> Option<i32>` — use `v.first()` and `map`. `double_first(&[21]) == Some(42)`, `double_first(&[]) == None`
   - `fn name_length(name: Option<&str>) -> usize` — `0` for `None`. Use `map_or`, or `map` + `unwrap_or`.
5. **Question:** `unwrap()` turns `None` into a crash (a _panic_). When is that acceptable? Why is `expect("reason")` better?

---

## Task 4 — Operators for the calculator

From the next lab on, we build a calculator. Its operators are an enum:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Op { Add, Sub, Mul, Div }
```

1. Implement on `Op`:
   - `fn from_char(c: char) -> Option<Op>` — `'+'`, `'-'`, `'*'`, `'/'`, otherwise `None`
   - `fn symbol(self) -> char` — the opposite direction
   - `fn precedence(self) -> u8` — `1` for `+ -`, `2` for `* /`
   - `fn apply(self, a: f64, b: f64) -> Option<f64>` — `None` when dividing by zero
2. `fn evaluate_simple(a: f64, op: char, b: f64) -> Option<f64>` — one line, using `?`.
   `evaluate_simple(6.0, '/', 4.0) == Some(1.5)`, `evaluate_simple(1.0, '/', 0.0) == None`, `evaluate_simple(1.0, '?', 2.0) == None`

---

## Task 5 — A tour of patterns

1. **Tuples and `_`** — `fn quadrant(p: (i32, i32)) -> &'static str` returning `"origin"`, `"on x axis"`, `"on y axis"` or `"elsewhere"`.
   _(`&'static str` is the type of a string literal. Lab 5 explains the `'static` part.)_
2. **`@` bindings** — `fn age_group(age: u32) -> String`: `n @ 0..=12` → `"child (7)"`, `13..=19` → `"teen (15)"`, otherwise `"adult (40)"`.
3. **Slice patterns** — `fn describe_list(v: &[i32]) -> String`: `[]` → `"empty"`, `[x]` → `"one: 4"`, `[first, .., last]` → `"from 1 to 3"`.

---

## Task 6 — A stack for numbers

```rust
#[derive(Debug)]
struct Stack { items: Vec<f64> }
```

1. `fn new() -> Self`, `fn push(&mut self, v: f64)`, `fn pop(&mut self) -> Option<f64>`, `fn peek(&self) -> Option<&f64>`, `fn len(&self) -> usize`.
2. `fn apply(&mut self, op: Op) -> Option<()>` — pops `b`, then `a`, and pushes `op.apply(a, b)`. If there are fewer than two values, or the operation fails, return `None` and leave the stack **unchanged**.
   _`Option<()>` means "succeeded, with no value to return" or "failed"._
3. Compute `(2 + 3) * 4 - 1` using only `push` and `apply`, then empty the stack with `while let Some(v) = stack.pop() { ... }`.
