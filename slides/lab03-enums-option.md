---
marp: true
theme: default
size: 4:3
paginate: false
---

<!-- _paginate: false -->

# Programming Languages

### Lab 3 - Enums and Option

Kacper Grabczak
[kgrabczak@swps.edu.pl](mailto:kgrabczak@swps.edu.pl)
GitHub:
[github.com/whnf](https://github.com/whnf)

---

# Algebraic Data Types (Haskell)

**Product**

```haskell
data Point = Point Int Int
```

**Coproduct (Sum)**

```haskell
data Bool = False | True
```

**Algebraic Data Types (ADTs)**: Coproducts of products

```haskell
data Shape = Circle Point Int | Rect Point Point
```

---

# Algebraic Data Types (Rust)

**Product**

```rust
struct Point(i32, i32);
```

**Coproduct (Sum)**

```rust
enum Bool { False, True }
```

**Algebraic Data Types (ADTs)**: Coproducts of products

```rust
enum Shape { Circle(Point, i32), Rect(Point, Point) }
```

---

# Enums and `match`

```rust
enum Light { Red, Yellow, Green }

fn next(light: Light) -> Light {
    match light {
        Light::Red => Light::Green,
        Light::Green => Light::Yellow,
        Light::Yellow => Light::Red,
    }
}
```

`match` must cover **every** case:

```
error[E0004]: non-exhaustive patterns
```

---

# Enums with data

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn show(m: &Message) -> String {
    match m {
        Message::Quit => "quit".to_string(),
        Message::Move { x, y } => format!("move {x},{y}"),
        Message::Write(text) => format!("write {text}"),
    }
}
```

---

# `Option` instead of `null`

> null: my billion-dollar mistake — Tony Hoare

```rust
enum Option<T> { None, Some(T) }

fn safe_div(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}
```

An `Option<f64>` is **not** an `f64`: you must check it first.

---

# Using `Option`

```rust
match safe_div(10.0, 2.0) {
    Some(r) => println!("result {r}"),  // result 5
    None => println!("division by zero"),
}

let r = safe_div(10.0, 0.0).unwrap_or(0.0);  // 0
```

`?` returns `None` early:

```rust
// a / b / c
fn div_twice(a: f64, b: f64, c: f64) -> Option<f64> {
    let x = safe_div(a, b)?;
    safe_div(x, c)
}
```
