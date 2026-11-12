---
marp: true
theme: default
size: 4:3
paginate: false
---

# Programming Languages

### Lab 6 — Closures, iterators and lifetimes

Kacper Grabczak
[kgrabczak@swps.edu.pl](mailto:kgrabczak@swps.edu.pl)
GitHub:
[github.com/whnf](https://github.com/whnf)

---

# Functions as values

**JavaScript**: an arrow function

```js
const addN = (x) => x + n;
```

**Haskell**: a lambda

```haskell
addN = \x -> x + n
```

**Rust**: a closure

```rust
let add_n = |x| x + n;
```

All three **capture** `n` from the surrounding scope.

---

# Closures

```rust
let inc = |x| x + 1;                  // types inferred
let inc = |x: i32| -> i32 { x + 1 };  // written out
```

A closure **borrows** what it captures. `move` makes it take **ownership** instead:

```rust
let s = String::from("hi");
let say = move || println!("{s}");
say();
println!("{s}");  // error: s was moved into say
```

---

# `Fn`, `FnMut`, `FnOnce`

What the closure does with what it captured:

| Trait    | Captured values are | Can be called           |
| -------- | ------------------- | ----------------------- |
| `Fn`     | only read           | many times              |
| `FnMut`  | changed             | many times, needs `mut` |
| `FnOnce` | given away          | once                    |

Every `Fn` is also `FnMut`, and every `FnMut` is also `FnOnce`.

---

# Closures as arguments

```rust
fn call_with_ten(f: impl Fn(i32) -> i32) -> i32 {
    f(10)
}

let n = 5;
call_with_ten(|x| x * n);  // 50
```

The bound says what `call_with_ten` needs:

- `impl FnOnce` — calls `f` at most once
- `impl FnMut` — calls `f` many times, `f` may change state
- `impl Fn` — calls `f` many times, `f` only reads

---

# Iterators

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

`Some(x)` is the next element, `None` means the end.

A `for` loop just calls `next` until `None`:

```rust
let mut it = vec![1, 2].into_iter();
it.next();  // Some(1)
it.next();  // Some(2)
it.next();  // None
```

---

# Adapters and consumers

```rust
let lens: Vec<usize> = "a bb ccc dddd"
    .split_whitespace()
    .map(|w| w.len())
    .filter(|&n| n > 1)
    .collect();  // [2, 3, 4]
```

| Adapters → iterator  | Consumers → value   |
| -------------------- | ------------------- |
| `map`, `filter`      | `sum`, `count`      |
| `filter_map`, `zip`  | `collect`, `find`   |
| `enumerate`, `take`  | `max`, `max_by_key` |
| `take_while`, `skip` | `any`, `all`        |
