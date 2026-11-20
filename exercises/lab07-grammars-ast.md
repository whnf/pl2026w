# Lab 7 — Grammars and syntax trees

---

## Task 1 — Peano numbers

Every natural number is either zero, or the **successor** (the next number) of a natural number:

```
<nat> ::= "Z" | "S(" <nat> ")"
```

`Z` is 0, `S(Z)` is 1, `S(S(Z))` is 2.

1. Define `enum Nat`, with one variant for each alternative of the grammar. The compiler will reject the first attempt: read the error, explain it, and fix it.
2. `fn from_u32(n: u32) -> Nat` and `fn to_u32(&self) -> u32`, both recursive. `Nat::from_u32(3).to_u32() == 3`.
3. `impl Display for Nat`, which prints the text from the grammar: `Nat::from_u32(2)` prints `S(S(Z))`.
4. `fn add(&self, other: &Nat) -> Nat` and `fn mul(&self, other: &Nat) -> Nat`, using only these rules, without converting to `u32`:
   - `Z + m = m`, and `S(n) + m = S(n + m)`
   - `Z · m = Z`, and `S(n) · m = m + n · m`

   `2 + 3 == 5`, `2 · 3 == 6`, `0 · 5 == 0`.

**Question:** try `Nat::from_u32(1_000_000)`. What happens, and why?

---

## Task 2 — Linked lists

A list is empty (`[]`), or a value in front of a list. This is how Haskell writes lists:

```
<list> ::= "[]" | <int> ":" <list>
```

`1:2:3:[]` is the list of 1, 2 and 3.

1. Define `enum List` for lists of `i32`. Then write `fn from_slice(values: &[i32]) -> List`, and `impl Display for List` in the form above: `List::from_slice(&[1, 2, 3])` prints `1:2:3:[]`, and an empty slice prints `[]`. _(Hint: a slice pattern `[first, rest @ ..]` splits a slice into its first element and the rest.)_
2. `fn len(&self) -> usize` and `fn sum(&self) -> i32`, both recursive: 3 and 6.
3. `fn map(&self, f: &impl Fn(i32) -> i32) -> List`: `list.map(&|x| x * 10)` prints `10:20:30:[]`. Why does `f` have to be a reference here?
4. `fn reverse(self) -> List`, with a loop instead of recursion: take the values from the front of the list one by one, and put each in front of a new list. `3:2:1:[]`.

**Questions**

- Compare `List` with `Nat` from Task 1. What is a Peano number, seen as a list?
- How long does it take to add a value at the front of a `List`? At the end? To read the 1000th value? Compare with `Vec`.

---

## Task 3 — Binary trees

A tree is empty (`.`), or a node with a left subtree, a value and a right subtree:

```
<tree> ::= "." | "(" <tree> " " <int> " " <tree> ")"
```

1. Define `enum Tree` for trees of `i32`. Write a helper `fn node(left: Tree, value: i32, right: Tree) -> Tree` that makes trees shorter to build, and build the tree `((. 1 .) 2 (. 3 .))`. Implement `Display` to print it in this form.
2. `fn size(&self) -> usize`, `fn depth(&self) -> usize` and `fn sum(&self) -> i32`. For the tree above: 3, 2 and 6.
3. A **binary search tree** keeps smaller values to the left of a node and larger ones to the right. Write `fn insert(self, value: i32) -> Tree`, which returns the tree with the value added (or unchanged, if it is already there), and `fn contains(&self, value: i32) -> bool`.
   Inserting `5, 3, 8, 1, 4` into an empty tree gives `(((. 1 .) 3 (. 4 .)) 5 (. 8 .))`. _(Hint: `fold`.)_
4. `fn to_vec(&self) -> Vec<i32>` — the values from left to right: `[1, 3, 4, 5, 8]`. Why are they sorted?

**Question:** insert `1, 2, 3, 4, 5`, in this order. What does the tree look like, and what is its depth? Why does it matter for `contains`?

---

## Task 4 — JSON

JSON is the most common format for sending data. Here is its grammar, a little simplified (no spaces, no escapes in strings):

```
<value>  ::= "null" | "true" | "false" | <number> | <string> | <array> | <object>
<array>  ::= "[" [ <value> { "," <value> } ] "]"
<object> ::= "{" [ <member> { "," <member> } ] "}"
<member> ::= <string> ":" <value>
```

1. Define `enum Json`. Use `f64` for numbers. Arrays and objects hold any number of values: which Rust type fits? Does `Json` need a `Box`, like the types in the previous tasks?
2. Build the value of this text by hand, and implement `Display`, which prints it back exactly like this:
   ```
   {"name":"Ada","langs":["Rust","Haskell"],"born":1815,"alive":false}
   ```
   _(Hint: build the parts with `map`, then `join(",")`.)_
3. `fn depth(&self) -> usize` — how deeply arrays and objects are nested: `0` for `null`, `true`, numbers and strings, `1 +` the largest depth inside for arrays and objects (`[]` has depth 1). The value above has depth 2.
4. `fn get(&self, key: &str) -> Option<&Json>` for objects and `fn index(&self, i: usize) -> Option<&Json>` for arrays; both return `None` for any other kind of value. Together: `value.get("langs")?.index(1)` is the string `"Haskell"`.

**Question:** an object could be stored as a `Vec` of pairs or as a `HashMap`. Which did you choose? What would change with the other one?

---

## Task 5 — The calculator's syntax tree

From the next lab on, we build a calculator: Lab 8 turns the text into tokens, and Lab 9 turns the tokens into this tree. Today we build trees by hand and evaluate them.

The calculator follows the usual rules: `*` and `/` bind tighter than `+` and `-`, `^` binds tighter than both and groups to the right (`2 ^ 3 ^ 2 = 2 ^ 9`), and unary minus binds looser than `^` (`-2 ^ 2 = -(2 ^ 2) = -4`).

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Op { Add, Sub, Mul, Div, Pow }

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Num(f64),
    Neg(Box<Expr>),
    Binary { op: Op, left: Box<Expr>, right: Box<Expr> },
}
```

1. `fn symbol(self) -> char` and `fn apply(self, a: f64, b: f64) -> Option<f64>` on `Op`, as in Homework 2, plus `Pow` (`powf`).
2. `Expr::num(n)`, `Expr::neg(e)` and `Expr::binary(op, left, right)`, which hide the `Box`es. Build the trees for `1 + 2 * 3`, `(1 + 2) * 3`, `8 - 4 - 2` and `-2 ^ 2`.
3. `enum EvalError { DivisionByZero }` and `fn eval(&self) -> Result<f64, EvalError>` (Lab 4). The trees from 5.2 give `7`, `9`, `2` and `-4`; `1 / (2 - 2)` gives `Err(DivisionByZero)`.
4. `impl Display for Expr` with **every** binary operation in parentheses: `(1 + (2 * 3))`, `((8 - 4) - 2)`, `-(2 ^ 2)`. In Lab 9 this shows how the parser read an expression.
5. `fn to_pretty(&self) -> String` with only the parentheses that are needed: `1 + 2 * 3`, `(1 + 2) * 3`, `8 - 4 - 2`, `8 - (4 - 2)`, `2 ^ 3 ^ 2`, `-2 ^ 2`, `-(1 + 2)`.
   _(Hint: give each kind of node a precedence level. A child needs parentheses if its level is lower than its parent's, or equal but on the side the operator does not group to.)_

**Question:** `(1 + 2) * 3` contains parentheses, but `Expr` has no variant for them. Why is none needed?
