---
marp: true
theme: default
size: 4:3
paginate: false
---

# Programming Languages

### Lab 5 — Traits and generics

Kacper Grabczak
[kgrabczak@swps.edu.pl](mailto:kgrabczak@swps.edu.pl)
GitHub:
[github.com/whnf](https://github.com/whnf)

---

# Shared behaviour

**Java**: an interface

```java
interface Animal { String name(); }
```

**Haskell**: a type class

```haskell
class Animal a where
  name :: a -> String
```

**Rust**: a trait

```rust
trait Animal { fn name(&self) -> String; }
```

---

# Traits

```rust
trait Animal {
    fn name(&self) -> String;
    fn greet(&self) -> String {  // default method
        format!("I am {}", self.name())
    }
}

struct Dog;

impl Animal for Dog {
    fn name(&self) -> String { "a dog".to_string() }
}
```

Dog gets greet for free, or can override it.

---

# Static and dynamic dispatch

```rust
fn introduce(a: &impl Animal) {  // static
    println!("{}", a.greet());
}

fn introduce_dyn(a: &dyn Animal) {  // dynamic
    println!("{}", a.greet());
}
```

|         | `impl Animal`   | `dyn Animal`       |
| ------- | --------------- | ------------------ |
| Decided | when compiling  | while running      |
| How     | a copy per type | a table of methods |

---

# Standard traits

Operators and formatting are traits too:

| You write         | Rust calls       |
| ----------------- | ---------------- |
| `println!("{v}")` | `Display::fmt`   |
| `a + b`           | `Add::add(a, b)` |
| `-a`              | `Neg::neg(a)`    |
| `Vec2::from(...)` | `From::from`     |

---

# Deriving

`#[derive]` writes the implementation for you:

```rust
#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Date { year: u32, month: u32, day: u32 }
```

| Trait        | Gives you                |
| ------------ | ------------------------ |
| `Debug`      | `{:?}`                   |
| `Clone`      | `.clone()`               |
| `PartialEq`  | `==`, `!=`               |
| `PartialOrd` | `<`, `>`, field by field |

---

# Generics

One function for many types:

```rust
fn print_all<T: Display>(items: &[T]) {
    for x in items { println!("{x}"); }
}
```

- `T` is a **type parameter**
- `Display` is a **bound**: what `T` must be able to do

`print_all(&[1, 2, 3])`, `print_all(&["a", "b"])`

Generic types: `Vec<T>`, `Option<T>`, `HashMap<K, V>`
