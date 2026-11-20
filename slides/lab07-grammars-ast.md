---
marp: true
theme: default
size: 4:3
paginate: false
---

# Programming Languages

### Lab 7 — Grammars and syntax trees

Kacper Grabczak
[kgrabczak@swps.edu.pl](mailto:kgrabczak@swps.edu.pl)
GitHub:
[github.com/whnf](https://github.com/whnf)

---

# Grammars

A **grammar** says which texts belong to a language.

**BNF**: each rule names a part of the language and lists its alternatives, separated by `|`. Text in quotes is written literally.

```
<digit>  ::= "0" | "1" | "2" | "3" | "4"
           | "5" | "6" | "7" | "8" | "9"
<number> ::= <digit> | <digit> <number>
```

`<number>` refers to **itself**: a number is a digit, or a digit followed by a number.

---

# Derivations

A **derivation** replaces names with one of their alternatives until only text is left:

```
<number> → <digit> <number>
         → 4 <number>
         → 4 <digit>
         → 42
```

A text belongs to the language if some derivation produces it.

---

# Exercise: BNF

On paper:

1. Which of these are a `<number>`: `7`, `007`, `4 2`, `-1`, and the empty text?
2. Write the derivation of `2026`.
3. Change the grammar so that a number has no leading zeros: `0` and `107` are allowed, `007` is not.
   _(Hint: add a rule for the digits `1`–`9`.)_

---

# EBNF

Shorthands for common patterns:

| EBNF         | Means                  |
| ------------ | ---------------------- |
| `{ x }`      | zero or more `x`       |
| `[ x ]`      | an optional `x`        |
| `( a \| b )` | `a` or `b`, as a group |

```
<number>  ::= <digit> { <digit> }
<integer> ::= [ "-" ] <number>
```

---

# Exercise: EBNF

4. Write a grammar for names: a letter or `_`, followed by any number of letters, digits and `_`. Once in BNF, once in EBNF.
5. List every text of at most 4 characters that this grammar allows (`""` is the empty text). Which language is it?

```
<parens> ::= "" | "(" <parens> ")" <parens>
```

---

# From grammar to types

A grammar describes **text**. In Rust, the same structure becomes **types**.

| Grammar                      | Rust                      |
| ---------------------------- | ------------------------- |
| a rule                       | a type                    |
| alternatives `a \| b`        | the variants of an `enum` |
| a rule that refers to itself | `Box`                     |
| `[ x ]`                      | `Option`                  |
| `{ x }`                      | `Vec`                     |

---

# Syntax trees

A value of those types is a **syntax tree**: the structure of the text, without the punctuation.

```
1 + 2 * 3          (1 + 2) * 3

    +                    *
   / \                  / \
  1   *                +   3
     / \              / \
    2   3            1   2
```

The parentheses are gone: the **shape** of the tree says what goes together.
