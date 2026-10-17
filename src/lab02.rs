//! Lab 2 — Ownership and structs (exercises/lab02-ownership-structs.md)

pub fn run() {
    println!("--- Task 1 ---");
    task1();
    println!("--- Task 2 ---");
    task2();
    println!("--- Task 3 ---");
    task3();
}

/// Task 1 — Predict: move, copy, clone
fn task1() {
    // (a): prints "5 5". `i32` is Copy, so `b = a` copies the value.
    let a = 5;
    let b = a;
    println!("{a} {b}");

    // (b): error[E0382]. `s1` was moved into `s2` and cannot be used any more.
    // A String is (pointer, length, capacity) on the stack plus a buffer on the
    // heap. A plain copy would give two owners of one buffer, and both would
    // free it (a double free), so a String is moved instead.
    // let s1 = String::from("hi");
    // let s2 = s1;
    // println!("{s1} {s2}");

    // (c): prints "hi hi". `clone` makes a deep copy of the heap buffer.
    let s1 = String::from("hi");
    let s2 = s1.clone();
    println!("{s1} {s2}");

    // (d): error[E0382]. `s` was moved into `take` and dropped when `take`
    // returned.
    // fn take(s: String) -> usize { s.len() }
    // let s = String::from("hello");
    // let n = take(s);
    // println!("{n} {s}");

    // (e): prints "(1, 2.0, 'x') (1, 2.0, 'x')". A tuple of Copy types is Copy.
    let t = (1, 2.0, 'x');
    let u = t;
    println!("{t:?} {u:?}");

    // (f): error[E0382]. `for x in v` consumes `v`. Use `for x in &v` to borrow.
    let v = vec![1, 2, 3];
    for x in &v {
        print!("{x} ");
    }
    println!("{}", v.len());
}

/// Task 2 — Fix the borrow checker
fn task2() {
    // (a): `print_len` only reads the string, so it borrows it.
    fn print_len(s: &str) {
        println!("{}", s.len());
    }
    let s = String::from("hello");
    print_len(&s);
    println!("{s}");

    // (b): `push` may move the data to a bigger buffer and free the old one,
    // so `first` would point to freed memory. `i32` is Copy, so copy the value
    // out and `first` no longer borrows `v`.
    let mut v = vec![1, 2, 3];
    let first = v[0];
    v.push(4);
    println!("first = {first}");

    // (c): `&g` would point to `g`, which is dropped when the function returns.
    // Return the owned String instead.
    fn make_greeting(name: &str) -> String {
        format!("Hello, {name}!")
    }
    println!("{}", make_greeting("Ada"));

    // (d): two `&mut` borrows must not overlap. A borrow ends after its last
    // use, so finish with `r1` before creating `r2`.
    let mut s = String::from("abc");
    let r1 = &mut s;
    r1.push('d');
    let r2 = &mut s;
    r2.push('e');
    println!("{s}");

    // (e): the function modifies the string, so it needs `&mut String`.
    fn add_exclamation(s: &mut String) {
        s.push('!');
    }
    let mut s = String::from("wow");
    add_exclamation(&mut s);
    println!("{s}");
}

/// Task 3 — Strings and slices
fn task3() {
    // 3.1: `split` always yields at least one piece, so `unwrap` cannot panic.
    // The result is a slice of `s`, so no new string is created.
    fn first_word(s: &str) -> &str {
        s.split(' ').next().unwrap()
    }

    // 3.2
    fn count_vowels(s: &str) -> usize {
        s.chars()
            .filter(|c| "aeiou".contains(c.to_ascii_lowercase()))
            .count()
    }

    // 3.3
    fn reverse_words(s: &str) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }

    // 3.4: `chars.as_str()` is the rest of the string after the first char.
    // `to_uppercase` returns several chars for some letters ('ß' -> "SS").
    fn capitalize(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    }

    // 3.5
    fn is_palindrome(s: &str) -> bool {
        let cleaned: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .flat_map(|c| c.to_lowercase())
            .collect();
        let reversed: Vec<char> = cleaned.iter().rev().copied().collect();
        cleaned == reversed
    }

    println!("{}", first_word("hello world")); // hello
    println!("{}", first_word("single")); // single
    println!("{}", count_vowels("Programming")); // 3
    println!("{}", reverse_words("the quick brown fox")); // fox brown quick the
    println!("{}", capitalize("rust")); // Rust
    println!("{:?}", capitalize("")); // ""
    println!("{}", is_palindrome("A man, a plan, a canal: Panama")); // true
}
