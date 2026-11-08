//! Lab 5 — Traits and generics (exercises/lab05-traits-generics.md)

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
    println!("--- Task 6 ---");
    task6();
    println!("--- Task 7 ---");
    task7();
}

trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;
    fn describe(&self) -> String {
        // a default method: types get it for free
        format!("{} with area {:.2}", self.name(), self.area())
    }
}

// 1.1
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn name(&self) -> String {
        "circle".to_string()
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

struct Square {
    side: f64,
}

impl Shape for Square {
    fn name(&self) -> String {
        "square".to_string()
    }

    fn area(&self) -> f64 {
        self.side.powi(2)
    }

    // 1.2
    fn describe(&self) -> String {
        format!("square {}x{}", self.side, self.side)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

/// Task 1 — Traits
fn task1() {
    println!("{}", Circle { radius: 1.0 }.describe()); // circle with area 3.14
    println!("{}", Square { side: 2.0 }.describe()); // square 2x2
}

/// Task 2 — `impl Trait` and `dyn Trait`
fn task2() {
    // 2.1
    fn print_shape(s: &impl Shape) {
        println!("{}", s.describe());
    }

    // 2.2
    fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
        shapes.iter().fold(0.0, |acc, shape| acc + shape.area())
    }

    print_shape(&Circle { radius: 1.0 }); // circle with area 3.14
    print_shape(&Square { side: 2.0 }); // square 2x2

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 2.0 }),
    ];
    println!("{:.2}", total_area(&shapes)); // 7.14

    // 2.3: `impl Shape` stands for ONE concrete type that the compiler picks,
    // so a `Vec<impl Shape>` is a vector of circles or a vector of squares,
    // never both. `Box<dyn Shape>` has the same type for every shape.
    //
    // With the trait, a new SHAPE is one new struct and one `impl`, and no
    // existing code changes. A new OPERATION means adding a method to the
    // trait and to every `impl`. With the enum from Lab 3 it is the other way
    // round.
}

/// Task 3 — Standard traits
fn task3() {
    #[derive(Debug, Clone, Copy, PartialEq, Default)]
    struct Vec2 {
        x: f64,
        y: f64,
    }

    // 3.1
    impl std::fmt::Display for Vec2 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    // 3.2
    impl std::ops::Add for Vec2 {
        type Output = Vec2;

        fn add(self, rhs: Self) -> Self::Output {
            Vec2 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    // 3.3
    impl From<(f64, f64)> for Vec2 {
        fn from((x, y): (f64, f64)) -> Self {
            Vec2 { x, y }
        }
    }

    // 3.4
    impl std::ops::Neg for Vec2 {
        type Output = Vec2;

        fn neg(self) -> Self::Output {
            Vec2 {
                x: -self.x,
                y: -self.y,
            }
        }
    }

    // 3.5
    impl std::ops::Mul<f64> for Vec2 {
        type Output = Vec2;

        fn mul(self, rhs: f64) -> Self::Output {
            Vec2 {
                x: self.x * rhs,
                y: self.y * rhs,
            }
        }
    }

    println!("{}", Vec2::from((1.0, 2.0)) + Vec2 { x: 0.5, y: 0.5 }); // (1.5, 2.5)
    println!("{}", -(Vec2::from((1.0, 2.0)) * 2.0)); // (-2, -4)
    println!("{}", Vec2::default()); // (0, 0)

    // The standard library has `impl<T, U: From<T>> Into<U> for T`: every
    // `From` gives the matching `Into` for free.
    let v: Vec2 = (1.0, 2.0).into();
    println!("{v}"); // (1, 2)
}

fn version(major: u32, minor: u32, patch: u32) -> Version {
    Version {
        major,
        minor,
        patch,
    }
}

/// Task 4 — Deriving comparisons
fn task4() {
    // 4.1: strings are compared character by character, and '1' < '9', so
    // "1.10.0" comes before "1.9.0". The derived order compares numbers.
    println!("{}", version(1, 10, 0) > version(1, 9, 3)); // true
    println!("{}", version(2, 0, 0) > version(1, 99, 99)); // true
    println!("{}", "1.10.0" > "1.9.0"); // false

    // 4.2: sorting has to compare any two elements, and `Ord` promises that
    // every pair has an answer (`PartialOrd` may say "not comparable").
    let mut versions = vec![version(1, 10, 0), version(0, 9, 1), version(1, 9, 3)];
    versions.sort();
    println!("{versions:?}"); // 0.9.1, 1.9.3, 1.10.0

    // 4.3
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum Level {
        Low,
        Medium,
        High,
    }

    println!("{}", Level::Low < Level::High); // true
    let mut levels = vec![Level::High, Level::Low, Level::Medium];
    levels.sort();
    println!("{levels:?}"); // [Low, Medium, High]

    // `f64::NAN` is not equal, less or greater than anything, not even itself,
    // so `f64` cannot promise that every pair has an order.
    println!("{:?}", f64::NAN.partial_cmp(&1.0)); // None
}

/// Task 5 — Generics
fn task5() {
    // 5.1: without `PartialOrd`, `item > max` fails: "binary operation `>`
    // cannot be applied to type `&T`". Without `Copy`, `Some(*max)` fails:
    // "cannot move out of `*max` which is behind a shared reference".
    fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
        if let [] = items {
            return None;
        }

        let mut max = items.first().unwrap();

        for item in items {
            if item > max {
                max = item;
            }
        }

        Some(*max)
    }

    println!("{:?}", largest(&[3, 7, 2])); // Some(7)
    println!("{:?}", largest(&[1.5, -2.0])); // Some(1.5)
    println!("{:?}", largest(&['a', 'z'])); // Some('z')
    println!("{:?}", largest::<i32>(&[])); // None

    // `Version` derives both `PartialOrd` and `Copy`, so it works.
    let versions = [version(1, 10, 0), version(1, 9, 3)];
    println!("{:?}", largest(&versions)); // Some(Version { major: 1, minor: 10, patch: 0 })

    // 5.2
    struct Stack<T> {
        items: Vec<T>,
    }

    impl<T> Stack<T> {
        fn new() -> Self {
            Stack { items: vec![] }
        }

        fn push(&mut self, x: T) {
            self.items.push(x)
        }

        fn pop(&mut self) -> Option<T> {
            self.items.pop()
        }

        fn peek(&self) -> Option<&T> {
            self.items.last()
        }
    }

    let mut numbers = Stack::new();
    numbers.push(1.0);
    numbers.push(2.0);
    println!("{:?}", numbers.pop()); // Some(2.0)
    println!("{:?}", numbers.peek()); // Some(1.0)

    let mut words = Stack::new();
    words.push("hello".to_string());
    println!("{:?}", words.peek()); // Some("hello")
    println!("{:?}", words.pop()); // Some("hello")
    println!("{:?}", words.pop()); // None

    // 5.3
    fn print_shape<T: Shape>(s: &T) {
        println!("{}", s.describe());
    }

    print_shape(&Square { side: 2.0 }); // square 2x2
}

/// Task 6 — Your trait for existing types
fn task6() {
    trait Double {
        fn double(&self) -> Self;
    }

    // 6.1
    impl Double for i32 {
        fn double(&self) -> Self {
            self * 2
        }
    }

    impl Double for String {
        fn double(&self) -> Self {
            format!("{self}{self}")
        }
    }

    // 6.2
    impl<T: Clone> Double for Vec<T> {
        fn double(&self) -> Self {
            let mut v = self.clone();
            v.append(&mut self.clone());
            v
        }
    }

    println!("{}", 21.double()); // 42
    println!("{}", "ab".to_string().double()); // abab
    println!("{:?}", vec![1, 2].double()); // [1, 2, 1, 2]

    // error[E0117]: the orphan rule allows an `impl` only if the trait or the
    // type is defined in our crate. `Display` and `Vec` both come from the
    // standard library. Otherwise two crates could each implement `Display`
    // for `Vec<i32>`, and the compiler would not know which one to use.
    // impl std::fmt::Display for Vec<i32> { ... }
}

/// Task 7 — `Option` under the hood
fn task7() {
    #[derive(Debug, PartialEq)]
    enum Maybe<T> {
        Nothing,
        Just(T),
    }

    // 7.1
    fn safe_div(a: f64, b: f64) -> Maybe<f64> {
        match b {
            0.0 => Maybe::Nothing,
            b => Maybe::Just(a / b),
        }
    }

    // 7.2
    impl<T> Maybe<T> {
        fn new(x: T) -> Self {
            Maybe::Just(x)
        }

        fn map<U>(self, f: fn(T) -> U) -> Maybe<U> {
            match self {
                Maybe::Nothing => Maybe::Nothing,
                Maybe::Just(x) => Maybe::Just(f(x)),
            }
        }

        fn flat_map<U>(self, f: fn(T) -> Maybe<U>) -> Maybe<U> {
            match self {
                Maybe::Nothing => Maybe::Nothing,
                Maybe::Just(x) => f(x),
            }
        }
    }

    fn double(x: f64) -> f64 {
        2.0 * x
    }

    fn reciprocal(x: f64) -> Maybe<f64> {
        safe_div(1.0, x)
    }

    println!("{:?}", safe_div(1.0, 4.0).map(double)); // Just(0.5)
    println!("{:?}", safe_div(1.0, 0.0).map(double)); // Nothing
    println!("{:?}", safe_div(1.0, 4.0).flat_map(reciprocal)); // Just(4.0)
    println!("{:?}", Maybe::new(0.0).flat_map(reciprocal)); // Nothing

    // 7.3
    for x in [0.0, 2.0, 4.0] {
        println!("{}", Maybe::new(x).flat_map(reciprocal) == reciprocal(x)); // true
        println!("{}", reciprocal(x).flat_map(Maybe::new) == reciprocal(x)); // true
    }

    // `map(reciprocal)` wraps the result again: Just(Just(4.0)). `flat_map`
    // does not add the second layer, so the steps can be chained.
    println!("{:?}", safe_div(1.0, 4.0).map(reciprocal)); // Just(Just(4.0))

    // `?` also stops at the first `None`. It does not work on `Maybe`: it
    // needs the `Try` trait, which is not stable yet, so only the standard
    // types (`Option`, `Result`, ...) support it.
}
