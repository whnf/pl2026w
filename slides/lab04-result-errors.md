---
marp: true
theme: default
size: 4:3
paginate: false
---

# Programming Languages

### Lab 4 — Result and errors

Kacper Grabczak
[kgrabczak@swps.edu.pl](mailto:kgrabczak@swps.edu.pl)
GitHub:
[github.com/whnf](https://github.com/whnf)

---

# How do we report errors?

**C**: a special return value

```c
FILE *f = fopen("data.txt", "r");  // NULL, reason in errno
```

**Java, Python**: exceptions

```python
int("abc")  # raises ValueError, not visible in the signature
```

**Rust**: the error is part of the **return type**

```rust
"abc".parse::<i32>()  // Result<i32, ParseIntError>
```

---

# `Result`

```rust
enum Result<T, E> { Ok(T), Err(E) }
```

Like `Option`, but the failure says **what** went wrong:

```rust
#[derive(Debug, PartialEq)]
enum CalcError { StackUnderflow, DivisionByZero }

fn div(a: f64, b: f64) -> Result<f64, CalcError> {
    if b == 0.0 { Err(CalcError::DivisionByZero) } else { Ok(a / b) }
}

match div(1.0, 0.0) {
    Ok(r) => println!("= {r}"),
    Err(e) => println!("error: {e:?}"),  // error: DivisionByZero
}
```

---

# `?` with `Result`

```rust
let x = div(a, b)?;
```

means

```rust
let x = match div(a, b) {
    Ok(v) => v,
    Err(e) => return Err(e),
};
```

The first error stops the function and goes to the caller.

---

# The unit type `()`

`()` is a type with **one** value, also written `()`.

```rust
fn greet() { println!("hi"); }  // returns ()
fn greet() -> () { println!("hi"); }  // the same
```

It says "nothing interesting to return":

```rust
fn push(&mut self, x: f64)  // returns ()
fn apply(&mut self, op: Op) -> Result<(), CalcError>
```

`()` is the empty tuple: a product of no types, like 1 is an empty product.

---

# `Result<T, ()>` ≅ `Option<T>`

`Err(())` carries no information, so it is just `None`:

```rust
fn to_option<T>(r: Result<T, ()>) -> Option<T> {
    match r { Ok(x) => Some(x), Err(()) => None }
}

fn to_result<T>(o: Option<T>) -> Result<T, ()> {
    match o { Some(x) => Ok(x), None => Err(()) }
}
```

The two functions undo each other: an **isomorphism**.
In Haskell: `Either () a` ≅ `Maybe a`.

Built in: `r.ok()` and `o.ok_or(())`.

---

# Tests first

**TDD**: red → green → refactor

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn division_by_zero() {
        assert_eq!(div(1.0, 0.0), Err(CalcError::DivisionByZero));
    }
}
```

```
$ cargo test
test tests::division_by_zero ... ok
```
