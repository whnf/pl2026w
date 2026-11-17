//! Lab 6 — Closures, iterators and lifetimes (exercises/lab06-closures-iterators.md)

pub fn run() {
    println!("--- Task 1 ---");
    task1();
    println!("--- Task 2 ---");
    task2();
    println!("--- Task 3 ---");
    task3();
    println!("--- Task 4 ---");
    task4();
    println!("--- Task 5 ---");
    task5();
}

/// Task 1 — Closures
fn task1() {
    // 1.1: without `move` the closure would borrow `n`, a local variable of
    // `make_adder` that is dropped when the function returns. `move` puts a
    // copy of `n` inside the closure.
    fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
        move |m| m + n
    }

    println!("{}", make_adder(10)(5)); // 15

    // 1.2: the closure changes the captured `c`, so it needs `&mut self` to be
    // called: that is `FnMut`. `Fn` closures may only read what they capture.
    fn make_counter() -> impl FnMut() -> u32 {
        let mut c = 0;

        move || {
            c += 1;
            c
        }
    }

    let mut counter = make_counter();
    println!("{} {} {}", counter(), counter(), counter()); // 1 2 3

    // 1.3
    // `a` only reads `name`: Fn.
    let name = String::from("Ada");
    let a = || println!("{name}");
    a(); // Ada
    a(); // Ada

    // `b` changes `count`: FnMut.
    let mut count = 0;
    let mut b = || count += 1;
    b();
    b();
    println!("{count}"); // 2

    // `c` gives away the String it owns, so it can run only once: FnOnce.
    // The second call fails with error[E0382]: use of moved value: `c`.
    let title = String::from("Dr");
    let c = move || title;
    println!("{}", c()); // Dr
    // c();
}

/// Task 2 — Passing closures to functions
fn task2() {
    // 2.1
    fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
        f(f(x))
    }

    println!("{}", apply_twice(|x| x * 3, 2)); // 18

    // 2.2: `b.1.cmp(&a.1)` (b before a) sorts the scores in descending order.
    let mut scores = vec![
        ("bob".to_string(), 80),
        ("ann".to_string(), 95),
        ("cid".to_string(), 80),
    ];
    scores.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    println!("{scores:?}"); // [("ann", 95), ("bob", 80), ("cid", 80)]

    // 2.3: a `fn` pointer is only a function, with no place to keep `k`, so a
    // closure that captures something cannot be one. `FnOnce` is enough
    // because `map` calls `f` at most once, and it accepts every closure.
    #[derive(Debug, PartialEq)]
    enum Maybe<T> {
        Nothing,
        Just(T),
    }

    impl<T> Maybe<T> {
        fn map<U>(self, f: impl FnOnce(T) -> U) -> Maybe<U> {
            match self {
                Maybe::Nothing => Maybe::Nothing,
                Maybe::Just(x) => Maybe::Just(f(x)),
            }
        }
    }

    fn safe_div(a: f64, b: f64) -> Maybe<f64> {
        match b {
            0.0 => Maybe::Nothing,
            b => Maybe::Just(a / b),
        }
    }

    let k = 3.0;
    println!("{:?}", safe_div(1.0, 4.0).map(|x| x * k)); // Just(0.75)
}

/// Task 3 — Iterator pipelines
fn task3() {
    use std::num::ParseIntError;

    // 3.1: a square is even exactly when the number is even.
    fn sum_even_squares(v: &[i32]) -> i32 {
        v.iter()
            .map(|n| n.pow(2))
            .filter(|n| n % 2 == 0)
            .fold(0, |acc, n| acc + n)
    }

    // 3.2
    fn parse_valid(s: &str) -> Vec<i32> {
        s.split_whitespace()
            .filter_map(|w| w.parse().ok())
            .collect()
    }

    // 3.3: collecting `Result`s into a `Result<Vec<_>, _>` stops at the first
    // `Err` and returns it.
    fn parse_all(s: &str) -> Result<Vec<i32>, ParseIntError> {
        s.split_whitespace().map(|w| w.parse()).collect()
    }

    // 3.4: zip([a, b, c], [x, y, z]) = [(a, x), (b, y), (c, z)]
    fn dot(a: &[i32], b: &[i32]) -> i32 {
        a.iter().zip(b).fold(0, |acc, (m, n)| acc + m * n)
    }

    // 3.5
    fn argmax(v: &[i32]) -> Option<(usize, i32)> {
        v.iter().copied().enumerate().max_by_key(|&(_, x)| x)
    }

    println!("{}", sum_even_squares(&[1, 2, 3, 4])); // 20
    println!("{:?}", parse_valid("1 x 2 3y 4")); // [1, 2, 4]
    println!("{:?}", parse_all("1 2 3")); // Ok([1, 2, 3])
    println!("{:?}", parse_all("1 2 x")); // Err(ParseIntError { kind: InvalidDigit })
    println!("{}", dot(&[1, 2, 3], &[4, 5, 6])); // 32
    println!("{:?}", argmax(&[3, 9, 2])); // Some((1, 9))
    println!("{:?}", argmax(&[])); // None

    // `iter()` yields `&T` and `iter_mut()` yields `&mut T`; the collection
    // stays usable. `into_iter()` yields `T` and consumes the collection.

    // Building the chain does no work. The numbers are computed one by one
    // only when something asks for them, here `collect`. It stops after 5.
    let v: Vec<i32> = (1..)
        .map(|x| x * x)
        .filter(|x| x % 3 == 1)
        .take(5)
        .collect();
    println!("{v:?}"); // [1, 4, 16, 25, 49]
    // Without `take(5)`, `collect` never stops asking: in a debug build `x * x`
    // soon overflows and panics, in a release build it runs out of memory.
}

/// Task 4 — Implementing `Iterator`
fn task4() {
    // 4.1
    struct Countdown(u32);

    impl Iterator for Countdown {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            match self.0 {
                0 => None,
                n => {
                    self.0 -= 1;
                    Some(n)
                }
            }
        }
    }

    println!("{:?}", Countdown(3).collect::<Vec<_>>()); // [3, 2, 1]
    println!("{}", Countdown(4).sum::<u32>()); // 10

    // 4.2: `None` in a field means "past the end of u64". This way the last
    // number that fits is still returned before the sequence ends.
    struct Fibonacci {
        curr: Option<u64>,
        next: Option<u64>,
    }

    impl Fibonacci {
        fn new() -> Self {
            Fibonacci {
                curr: Some(0),
                next: Some(1),
            }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            let curr = self.curr?;
            self.curr = self.next;
            self.next = self.next.and_then(|n| n.checked_add(curr));
            Some(curr)
        }
    }

    println!("{:?}", Fibonacci::new().take(10).collect::<Vec<_>>()); // [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
    let even_sum: u64 = Fibonacci::new()
        .take_while(|&n| n < 4_000_000)
        .filter(|n| n % 2 == 0)
        .sum();
    println!("{even_sum}"); // 4613732
    println!("{}", Fibonacci::new().count()); // 94
}

/// Task 5 — Lifetimes
fn task5() {
    // 5.1: without `'a`: error[E0106]: missing lifetime specifier. The result
    // borrows from `a` or `b`, and the compiler cannot tell which. With `'a`
    // the signature promises that the result is valid as long as BOTH `a` and
    // `b` are.
    fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
        if a.len() >= b.len() { a } else { b }
    }

    println!("{}", longest("long string", "xyz")); // long string

    // 5.2: error[E0597]: `b` does not live long enough. `b` is dropped at the
    // end of the inner block, but `result` is used after it. The compiler
    // only reads the signature: `result` may point to `b`, so it must not
    // outlive `b`. That here it would point to `a` does not matter.
    // let result;
    // let a = String::from("long string");
    // {
    //     let b = String::from("xyz");
    //     result = longest(&a, &b);
    // }
    // println!("{result}");

    // 5.3
    struct Excerpt<'a> {
        text: &'a str,
    }

    impl Excerpt<'_> {
        fn word_count(&self) -> usize {
            self.text.split_whitespace().count()
        }
    }

    fn first_sentence(text: &str) -> Excerpt<'_> {
        match text.find('.') {
            Some(i) => Excerpt { text: &text[..=i] },
            None => Excerpt { text },
        }
    }

    let excerpt = first_sentence("Call me Ishmael. Some years ago...");
    println!("{} ({} words)", excerpt.text, excerpt.word_count()); // Call me Ishmael. (3 words)

    // 5.4: every word is a slice of the original string, so nothing is copied.
    struct Words<'a> {
        rest: &'a str,
    }

    impl<'a> Iterator for Words<'a> {
        type Item = &'a str;

        fn next(&mut self) -> Option<Self::Item> {
            self.rest = self.rest.trim_start();
            if self.rest.is_empty() {
                return None;
            }
            let end = self.rest.find(' ').unwrap_or(self.rest.len());
            let word = &self.rest[..end];
            self.rest = &self.rest[end..];
            Some(word)
        }
    }

    let words = Words {
        rest: "  the quick  brown fox ",
    };
    println!("{:?}", words.collect::<Vec<_>>()); // ["the", "quick", "brown", "fox"]
}
