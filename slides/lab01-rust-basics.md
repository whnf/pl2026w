---
marp: true
theme: default
size: 4:3
paginate: true
---

<!-- _paginate: false -->

# Programming Languages

### Lab 1 — Rust basics

Kacper Grabczak
[kgrabczak@swps.edu.pl](mailto:kgrabczak@swps.edu.pl)
GitHub:
[github.com/whnf](https://github.com/whnf)

---

# What these classes are about

- The Rust language
- Making an interpreter

---

# Install Rust

1. **rustup** — installs `rustc`, `cargo`, `rustfmt`, `clippy`
   - Visit **[rustup.rs](https://rustup.rs)**
   - Install from instructions
   - Check: `rustc --version` and `cargo --version`
2. **Editor** (pick one)
   - **VS Code** + the **rust-analyzer** extension
   - **RustRover** (JetBrains, free for non-commercial use)

---

# Create and run a project

```bash
cargo new hello
cd hello
cargo run
```

**Cargo creates:**

```
hello/
├── Cargo.toml     # project name, version, dependencies
└── src/
    └── main.rs    # fn main() { println!("Hello, world!"); }
```

`cargo run` compiles the project and runs it
