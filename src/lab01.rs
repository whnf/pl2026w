//! Lab 1 — First Steps in Rust (exercises/lab01-rust-basics-exercises.md)

// 2.5: a `const` must have an explicit type, is evaluated at compile time,
// can live outside functions, and is inlined wherever it is used.
// An immutable `let` is a runtime value that only exists inside its scope.
const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("--- Task 1 ---");
    task1();
    println!("--- Task 2 ---");
    task2();
    println!("--- Task 3 ---");
    task3();
}

/// Task 1 — Formatting output
fn task1() {
    let name = "Ada";
    let age = 36;
    println!("Name: {name}, age: {age}");

    let pi = std::f64::consts::PI;
    println!("Pi to 3 places: {pi:.3}");

    // `>` right-aligns, `<` left-aligns, `^` centres, all within a width of 8.
    println!("|{:>8}|{:<8}|{:^8}|", "right", "left", "center");

    let triple = (1, "two", 3.0);
    println!("Debug: {triple:?}");

    println!("0x {:x}, 0b {:b}", 255, 10);
}

/// Task 2 — Variables, mutability and shadowing
fn task2() {
    // 2.1: prints "inner: 12" then "outer: 6". Each `let x` creates a *new*
    // variable that hides the old one, so nothing is mutated. The inner
    // shadow ends with its block, and the outer `x` (6) is visible again.
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("inner: {x}");
    }
    println!("outer: {x}");

    // 2.2
    let mut count = 0;
    count += 1;
    println!("{count}");

    // 2.3: shadowing lets the same name change type: &str -> &str -> u32.
    let input = "  42  ";
    let input = input.trim();
    let input: u32 = input.parse().expect("input should be a number");
    println!("parsed: {input}");

    // 2.4: `let mut` keeps one variable (same type) whose value changes over
    // time, e.g. a counter in a loop. Shadowing creates a new, still immutable
    // variable and may change the type, which suits step-by-step
    // transformations like the parsing above.

    println!("MAX_POINTS = {MAX_POINTS}");
}

/// Task 3 — Scalar types and integer overflow
fn task3() {
    // 3.1
    println!("i8::MIN = {}, i8::MAX = {}", i8::MIN, i8::MAX);
    println!("u8::MAX = {}, i32::MAX = {}", u8::MAX, i32::MAX);
    println!("u64::MAX = {}", u64::MAX);
    println!("f64::EPSILON = {}", f64::EPSILON);

    // 3.2: in a debug build `255 + 1` panics ("attempt to add with overflow");
    // in a release build the check is removed for speed and the value wraps
    // to 0. With literal values like these the compiler spots the overflow
    // and refuses to build at all (the `arithmetic_overflow` lint).
    // let x: u8 = 255;
    // let y = x + 1;
    // println!("y = {y}");

    // 3.3
    let x: u8 = 250;
    println!("checked_add: {:?}", x.checked_add(10)); // None
    println!("wrapping_add: {}", x.wrapping_add(10)); // 4
    println!("saturating_add: {}", x.saturating_add(10)); // 255
    println!("overflowing_add: {:?}", x.overflowing_add(10)); // (4, true)

    // 3.4
    fn factorial(n: u32) -> Option<u64> {
        let mut result: u64 = 1;
        for i in 1..=u64::from(n) {
            // `?` returns `None` from the function as soon as a step overflows.
            result = result.checked_mul(i)?;
        }
        Some(result)
    }

    println!("5! = {:?}", factorial(5));
    println!("20! = {:?}", factorial(20));
    println!("21! = {:?}", factorial(21));

    // 3.5: `as` never fails. It silently truncates, wraps or saturates.
    println!("300_i32 as u8 = {}", 300_i32 as u8); // 44 (keeps the low 8 bits)
    println!("-1_i32 as u32 = {}", -1_i32 as u32); // 4294967295 (same bits)
    println!("3.99_f64 as i32 = {}", 3.99_f64 as i32); // 3 (truncates towards 0)
    println!("-3.99_f64 as u32 = {}", -3.99_f64 as u32); // 0 (saturates)
    // `try_from` reports a value that does not fit instead of changing it.
    println!("u8::try_from(300) = {:?}", u8::try_from(300_i32)); // Err(..)
    println!("u8::try_from(200) = {:?}", u8::try_from(200_i32)); // Ok(200)
}
