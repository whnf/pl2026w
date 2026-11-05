# Lab 5 — Traits and generics

---

## Task 1 — Traits

A **trait** lists methods that a type promises to have, like an interface in Java.

```rust
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;
    fn describe(&self) -> String {   // a default method: types get it for free
        format!("{} with area {:.2}", self.name(), self.area())
    }
}
```

1. Implement `Shape` for `struct Circle { r: f64 }` and `struct Square { side: f64 }`.
   `Circle { r: 1.0 }.describe() == "circle with area 3.14"`
2. In `Square`, **override** `describe` to return `"square 2x2"` (for side 2).

---

## Task 2 — `impl Trait` and `dyn Trait`

1. Write `fn print_shape(s: &impl Shape)`, which prints `s.describe()`. It accepts any type that implements `Shape`. The compiler makes a separate copy of the function for each type: **static dispatch**.
2. Write `fn total_area(shapes: &[Box<dyn Shape>]) -> f64`, and call it with a `Vec<Box<dyn Shape>>` holding one circle and one square. Each `Box<dyn Shape>` finds the right `area` while the program runs: **dynamic dispatch**.
   `total_area` of a circle with `r = 1` and a square with `side = 2` is `≈ 7.14`.
3. **Questions:**
   - Why can't a `Vec<impl Shape>` hold both a `Circle` and a `Square`?
   - In Lab 3, `Shape` was an **enum**. Now it is a **trait**. Which one makes it easier to add a new shape? To add a new operation?

---

## Task 3 — Standard traits

The standard library defines traits that make your types work with the rest of the language: `{}` uses `Display` (Lab 4), `+` uses `Add`, `.into()` uses `From`.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Vec2 { x: f64, y: f64 }
```

Implement:

1. `std::fmt::Display` — `println!("{v}")` prints `(1, 2)`
2. `std::ops::Add` — `a + b`
3. `From<(f64, f64)>` — `Vec2::from((1.0, 2.0))`
4. `std::ops::Neg` — `-v`
5. `std::ops::Mul<f64>` — `v * 2.0` scales both coordinates

`Vec2::from((1.0, 2.0)) + Vec2 { x: 0.5, y: 0.5 }` prints `(1.5, 2.5)`; `-(Vec2::from((1.0, 2.0)) * 2.0)` prints `(-2, -4)`; `Vec2::default()` is `(0, 0)`.

**Question:** you only implemented `From`, yet `let v: Vec2 = (1.0, 2.0).into();` works too. Why?

---

## Task 4 — Deriving comparisons

`#[derive]` can also write comparisons. A derived `PartialOrd` compares the fields **in order**, like words in a dictionary.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Version { major: u32, minor: u32, patch: u32 }
```

1. Writing `1.10.0` for `Version { major: 1, minor: 10, patch: 0 }`, check that `1.10.0 > 1.9.3` and `2.0.0 > 1.99.99`. Then compare the strings `"1.10.0"` and `"1.9.0"`. Why does the string comparison give the wrong answer?
2. Sort a vector of `1.10.0`, `0.9.1` and `1.9.3` with `.sort()`. Why does `sort` need `Ord`?
3. Enums can be compared too: the variants are ordered as they are written.
   ```rust
   #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
   enum Level { Low, Medium, High }
   ```
   `Level::Low < Level::High`. Sort `vec![High, Low, Medium]`.

**Question:** `f64` has `PartialOrd` but not `Ord`. Why? _(Hint: `f64::NAN`.)_

---

## Task 5 — Generics

1. `fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T>` — must work for `i32`, `f64` and `char`.
   `largest(&[3, 7, 2]) == Some(7)`, `largest(&['a', 'z']) == Some('z')`, `largest::<i32>(&[]) == None`
   _What happens if you remove `PartialOrd` from the bound? And `Copy`?_
   Does `largest` work on a slice of `Version`s from Task 4? What does it need?
2. The stack from Lab 4 only holds `f64`. Write a **generic** `Stack<T>` with `new`, `push`, `pop -> Option<T>` and `peek -> Option<&T>`. Use it with `f64` and with `String`.
3. `fn print_shape(s: &impl Shape)` from Task 2 is a short way to write `fn print_shape<T: Shape>(s: &T)`. Rewrite it in this form.

---

## Task 6 — Your trait for existing types

You can implement **your own** trait for types you did not write, even for `i32`.

```rust
trait Double {
    fn double(&self) -> Self;
}
```

1. Implement `Double` for `i32` and for `String`: `21.double() == 42`, `"ab".to_string().double() == "abab"`.
2. Implement it once for **every** vector whose elements can be cloned: `impl<T: Clone> Double for Vec<T>`. `vec![1, 2].double() == vec![1, 2, 1, 2]`.

**Question:** try `impl std::fmt::Display for Vec<i32>`. Why is it rejected? _(Look up the "orphan rule".)_

---

## Task 7 — `Option` under the hood

`Option` is an ordinary enum from the standard library. Write your own, with Haskell's names:

```rust
#[derive(Debug, PartialEq)]
enum Maybe<T> { Nothing, Just(T) }
```

1. `fn safe_div(a: f64, b: f64) -> Maybe<f64>`, as in Lab 3 but with `Maybe`.
2. In `impl<T> Maybe<T>`:
   - `fn new(x: T) -> Maybe<T>` — puts a value inside: `Just(x)`
   - `fn map<U>(self, f: fn(T) -> U) -> Maybe<U>` — applies `f` to the value inside, if there is one
   - `fn flat_map<U>(self, f: fn(T) -> Maybe<U>) -> Maybe<U>` — the same, for an `f` that can fail itself; the result is not wrapped twice

   `f` is a function. With `fn double(x: f64) -> f64` and `fn reciprocal(x: f64) -> Maybe<f64>` (`Nothing` for `0`):
   - `safe_div(1.0, 4.0).map(double) == Just(0.5)`, `safe_div(1.0, 0.0).map(double) == Nothing`
   - `safe_div(1.0, 4.0).flat_map(reciprocal) == Just(4.0)`, `Maybe::new(0.0).flat_map(reciprocal) == Nothing`

   _(Lab 6 shows a shorter way to pass functions: closures.)_
3. With `new` and `flat_map`, `Maybe` is a **monad**. Check two of the monad laws for a few values:
   - `Maybe::new(x).flat_map(f) == f(x)`
   - `m.flat_map(Maybe::new) == m`

**Questions:**

- What would `safe_div(1.0, 4.0).map(reciprocal)` return? Why do we need `flat_map`?
- `flat_map` stops at the first `Nothing`. Which operator from Lab 3 does the same? Can you use it on a `Maybe`?

