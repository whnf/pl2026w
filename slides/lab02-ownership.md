---
marp: true
theme: default
size: 4:3
paginate: true
---

<!-- _paginate: false -->

# Programming Languages

### Lab 2 - Memory and ownership

Kacper Grabczak
[kgrabczak@swps.edu.pl](mailto:kgrabczak@swps.edu.pl)
GitHub:
[github.com/whnf](https://github.com/whnf)

---

# Who frees memory?

- **Stack**: local variables, freed when the function returns
- **Heap**: `String`, `Vec`, lives until someone frees it

Three answers:

- the programmer (C)
- a garbage collector (Java, Python, Go)
- RAII + ownership (C++, Rust)

---

# By hand (C)

```c
char *name = malloc(6);
strcpy(name, "Alice");
free(name);
printf("%s\n", name);   // use after free
```

The compiler checks nothing: leaks, double free, use after free.

About **70%** of serious security bugs in Chrome and Windows are memory bugs.

---

# Garbage collection

The runtime finds objects nothing points to and frees them.

- ✓ safe
- ✗ pauses and extra memory
- ✗ you don't know **when** memory is freed
- ✗ files and locks still need `try (...)` / `with`

---

# RAII (C++)

The destructor runs **at the end of the scope**:

```cpp
void write_log() {
    std::ofstream file("log.txt");  // opens
}                                   // closes
```

But C++ does not check references:

```cpp
std::vector<int> v = {1, 2, 3};
int &first = v[0];
v.push_back(4);      // may move the data
std::cout << first;  // reads freed memory
```

---

# Rust: ownership

1. Each value has **one owner**
2. The owner goes out of scope → the value is **dropped**
3. Many `&T` **or** one `&mut T`

```rust
let s1 = String::from("hi");
let s2 = s1;            // move: s2 is the owner
println!("{s1}");       // error[E0382]
```

One owner → no double free, no use after free.

---

# Rust: borrowing

The C++ bug, in Rust:

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];      // borrow
v.push(4);              // error[E0502]
println!("{first}");
```

`push` needs `&mut v` while `first` still reads `v`.
Rejected at compile time.

---

# Comparison

| | Safe | No runtime | Frees at a known point |
|---|---|---|---|
| **C** | ✗ | ✓ | ✓ |
| **GC** | ✓ | ✗ | ✗ |
| **C++** | ✗ | ✓ | ✓ |
| **Rust** | ✓ | ✓ | ✓ |

The price in Rust: the borrow checker.
