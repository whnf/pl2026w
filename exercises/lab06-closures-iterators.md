# Lab 6 — Closures, iterators and lifetimes

---

## Task 1 — Closures

A **closure** is an anonymous function that can **capture** variables from where it is defined:

```rust
let n = 10;
let add_n = |x| x + n;   // captures n
println!("{}", add_n(5));  // 15
```

1. `fn make_adder(n: i32) -> impl Fn(i32) -> i32` returns a closure adding `n`. You will need the `move` keyword — why?
   `make_adder(10)(5) == 15`
2. `fn make_counter() -> impl FnMut() -> u32` — each call returns the next number: 1, 2, 3, … Why `FnMut` and not `Fn`?
3. Rust has three closure traits. For each closure below, which is the most general one it implements: `Fn`, `FnMut` or `FnOnce`?
   ```rust
   let name = String::from("Ada");
   let a = || println!("{name}");
   let mut count = 0;
   let mut b = || count += 1;
   let title = String::from("Dr");
   let c = move || title;   // returns the captured String
   ```
   Check your answers: call each closure twice.

---

## Task 2 — Passing closures to functions

1. `fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32` — `apply_twice(|x| x * 3, 2) == 18`.
2. Sort a `Vec<(String, u32)>` of (name, score) pairs: by score **descending**, then by name **ascending**. Use `sort_by` with a closure, `cmp` and `then`.
   `[("bob", 80), ("ann", 95), ("cid", 80)]` → `[("ann", 95), ("bob", 80), ("cid", 80)]`
3. In Lab 5, `Maybe::map` took `f: fn(T) -> U`. Copy `Maybe` and `safe_div` and try:
   ```rust
   let k = 3.0;
   println!("{:?}", safe_div(1.0, 4.0).map(|x| x * k));
   ```
   Why is it rejected? Change the parameter to `f: impl FnOnce(T) -> U` so that it gives `Just(0.75)`. Why is `FnOnce` enough here?

---

## Task 3 — Iterator pipelines

Solve each one with a **single iterator chain**: no `for` loop, no `mut` variable.

| # | Function | Example |
|---|---|---|
| 1 | `sum_even_squares(v: &[i32]) -> i32` | `[1, 2, 3, 4] → 20` |
| 2 | `parse_valid(s: &str) -> Vec<i32>` — skip what is not a number (`filter_map`) | `"1 x 2 3y 4" → [1, 2, 4]` |
| 3 | `parse_all(s: &str) -> Result<Vec<i32>, ParseIntError>` — fail on the first bad item (`collect` into a `Result`) | `"1 2 x" → Err(..)` |
| 4 | `dot(a: &[i32], b: &[i32]) -> i32` (`zip`) | `[1, 2, 3]·[4, 5, 6] → 32` |
| 5 | `argmax(v: &[i32]) -> Option<(usize, i32)>` (`enumerate` + `max_by_key`) | `[3, 9, 2] → Some((1, 9))` |

**Questions**

- What is the difference between `iter()`, `iter_mut()` and `into_iter()`?
- Iterators are **lazy**. What does `(1..).map(|x| x * x).filter(|x| x % 3 == 1).take(5)` compute, and when does the work happen? What if you remove `.take(5)` and call `collect()`?

---

## Task 4 — Implementing `Iterator`

You only have to write one method, `next`. Every adapter (`map`, `filter`, `sum`, `zip`, …) then works for free.

1. `struct Countdown(u32)` yields `n, n-1, …, 1`. `Countdown(3).collect::<Vec<_>>() == [3, 2, 1]`, `Countdown(4).sum::<u32>() == 10`.
2. `struct Fibonacci` yields `0, 1, 1, 2, 3, 5, …`. Use `checked_add`, and **end** the sequence (return `None`) instead of overflowing.
   - the first 10 numbers → `[0, 1, 1, 2, 3, 5, 8, 13, 21, 34]`
   - the sum of the even Fibonacci numbers below 4 000 000 → `4613732` _(`take_while`)_
   - how many Fibonacci numbers fit in a `u64`? → `94`

---

## Task 5 — Lifetimes

A **lifetime** names how long a reference stays valid. Usually the compiler works it out alone; sometimes you have to say it.

1. This does not compile. Read the error, then add a lifetime `'a` so that it does:
   ```rust
   fn longest(a: &str, b: &str) -> &str {
       if a.len() >= b.len() { a } else { b }
   }
   ```
   What does the signature `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str` promise?
2. Explain why this is rejected, even though `longest` is now correct:
   ```rust
   let result;
   let a = String::from("long string");
   {
       let b = String::from("xyz");
       result = longest(&a, &b);
   }
   println!("{result}");
   ```
3. A struct that **holds** a reference needs a lifetime parameter:
   ```rust
   struct Excerpt<'a> { text: &'a str }
   ```
   Write `fn first_sentence(text: &str) -> Excerpt<'_>` (the text up to and including the first `.`, or all of it), and a method `fn word_count(&self) -> usize`. _(`'_` means "the lifetime the compiler infers". Written out in full: `fn first_sentence<'a>(text: &'a str) -> Excerpt<'a>`.)_
   `first_sentence("Call me Ishmael. Some years ago...").text == "Call me Ishmael."`
4. An iterator can hand out references into the data it walks over. Write `struct Words<'a> { rest: &'a str }` implementing `Iterator<Item = &'a str>`: each call to `next` returns the next word, without copying it.
   `Words { rest: "  the quick  brown fox " }` yields `"the"`, `"quick"`, `"brown"`, `"fox"`.
   _Hint: `trim_start`, then `find(' ')`, then split `rest` with slices._

---

## Task 6 — Memoization

A **memoized** function remembers its results: the second call with the same argument returns the stored result instead of computing it again.

1. Write a function that takes any function and returns a memoized version of it, keeping the results in a `HashMap` captured by the closure:
   ```rust
   use std::collections::HashMap;
   use std::hash::Hash;

   fn memoize<K: Eq + Hash + Copy, V: Clone>(f: impl Fn(K) -> V) -> impl FnMut(K) -> V
   ```
   Test it with a closure that prints when it really computes:
   ```rust
   let mut square = memoize(|n: u64| { println!("computing {n}"); n * n });
   square(4); square(4); square(5);   // prints "computing 4" and "computing 5" only
   ```
   _Hint: `cache.entry(k).or_insert_with(|| f(k))`._
   **Question:** why does `memoize` return `FnMut`, and not `Fn`?
2. Here is a slow function:
   ```rust
   fn slow_square(n: u64) -> u64 {
       std::thread::sleep(std::time::Duration::from_secs(1));
       n * n
   }
   ```
   Map `[2, 3, 2, 2, 3]` to `[4, 9, 4, 4, 9]`, once with `slow_square` and once with `memoize(slow_square)`. Time both with `std::time::Instant::now()` and `.elapsed()`. How many seconds does each take, and why?
   **Question:** would `memoize` speed up a recursive `fib(n)` that calls `fib(n - 1) + fib(n - 2)`? Why not?
