# Lab 2 — Ownership and structs

---

## Task 1 — Predict: move, copy, clone

For each snippet, decide whether it **compiles**. If it does, what does it print? If it doesn't, why not? Check your answers with the compiler.

```rust
// (a)
let a = 5;
let b = a;
println!("{a} {b}");

// (b)
let s1 = String::from("hi");
let s2 = s1;
println!("{s1} {s2}");

// (c)
let s1 = String::from("hi");
let s2 = s1.clone();
println!("{s1} {s2}");

// (d)
fn take(s: String) -> usize { s.len() }
let s = String::from("hello");
let n = take(s);
println!("{n} {s}");

// (e)
let t = (1, 2.0, 'x');
let u = t;
println!("{t:?} {u:?}");

// (f)
let v = vec![1, 2, 3];
for x in v { print!("{x} "); }
println!("{}", v.len());
```

---

## Task 2 — Fix the borrow checker

Each program below is rejected by the compiler. Read the **full** error message (`rustc --explain E0382` etc.), then fix the program **without** simply adding `.clone()` everywhere. Explain the fix in a comment.

```rust
// (a) E0382: borrow of moved value
fn print_len(s: String) { println!("{}", s.len()); }
fn main() {
    let s = String::from("hello");
    print_len(s);
    println!("{s}");
}
```

```rust
// (b) E0502: cannot borrow as mutable because it is also borrowed as immutable
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    println!("first = {first}");
}
```
_Why is this dangerous? What could `push` do to the memory `first` points to?_

```rust
// (c) E0515: cannot return reference to local variable — a dangling reference
fn make_greeting(name: &str) -> &String {
    let g = format!("Hello, {name}!");
    &g
}
```

```rust
// (d) E0499: cannot borrow as mutable more than once
fn main() {
    let mut s = String::from("abc");
    let r1 = &mut s;
    let r2 = &mut s;
    r1.push('d');
    r2.push('e');
    println!("{s}");
}
```

```rust
// (e) E0596: cannot borrow as mutable
fn add_exclamation(s: &String) { s.push('!'); }
```

---

## Task 3 — Strings and slices

`String` is an owned, growable, heap-allocated UTF-8 buffer. `&str` is a borrowed **view** into UTF-8 data (part of a `String`, or a literal). Functions that only read text should take `&str`. Then they accept both `&String` and literals.

Implement:

1. `fn first_word(s: &str) -> &str` — the text up to the first space, or the whole string.
   `first_word("hello world") == "hello"`, `first_word("single") == "single"`
2. `fn count_vowels(s: &str) -> usize` — count `aeiou`, ignoring case. Use `s.chars()`.
3. `fn reverse_words(s: &str) -> String` — `"the quick brown fox"` → `"fox brown quick the"`. Use `split_whitespace`.
4. `fn capitalize(s: &str) -> String` — upper-case the first character: `"rust" → "Rust"`, `"" → ""`.
5. `fn is_palindrome(s: &str) -> bool` — ignore case and anything that is not alphanumeric:
   `is_palindrome("A man, a plan, a canal: Panama")` is `true`.

---

## Task 4 — Vectors and slices

1. `fn running_sum(values: &[i32]) -> Vec<i32>` — `[1, 2, 3, 4]` → `[1, 3, 6, 10]`.
2. `fn double_in_place(values: &mut [i32])` — multiply every element by 2. Use `for x in values.iter_mut()` (or `for x in values`) and `*x *= 2`.
3. `fn remove_duplicates(values: &mut Vec<i32>)` — keep the first occurrence of each value and preserve the order: `[3, 1, 3, 2, 1]` → `[3, 1, 2]`. _(Hint: build a new vector and check `contains`, then assign it back with `*values = result`. Also look at `retain`.)_
4. `fn largest(values: &[i32]) -> Option<i32>` — `None` for an empty slice.

---

## Task 5 — Structs and methods

1. Define
   ```rust
   #[derive(Debug, Clone, Copy, PartialEq)]
   struct Point { x: f64, y: f64 }
   ```
   with an associated function `Point::new(x, y)`, a constant `Point::ORIGIN`, and a method `fn distance(&self, other: &Point) -> f64`.
   _What does each `derive` give you? Remove `Copy` and see what breaks._
2. Define
   ```rust
   struct Rectangle { top_left: Point, width: f64, height: f64 }
   ```
   with:
   - `fn new(top_left: Point, width: f64, height: f64) -> Self`
   - `fn square(top_left: Point, size: f64) -> Self` (associated function)
   - `fn area(&self) -> f64`
   - `fn contains(&self, p: Point) -> bool` (y grows downwards, like on a screen)
   - `fn can_hold(&self, other: &Rectangle) -> bool` (compare sizes only)
   - `fn scale(&mut self, factor: f64)`
