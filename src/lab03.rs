//! Lab 3 — Enums and Option (exercises/lab03-enums-option.md)

pub fn run() {
    println!("--- Task 1 ---");
    task1();
    println!("--- Task 2 ---");
    task2();
    println!("--- Task 3 ---");
    task3();
    println!("--- Task 5 ---");
    task5();
}

/// Task 1 — Simple enums
fn task1() {
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    // 1.1
    impl Direction {
        fn turn_right(self) -> Self {
            match self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            }
        }

        fn opposite(self) -> Self {
            self.turn_right().turn_right()
        }

        fn turn_left(self) -> Self {
            self.opposite().turn_right()
        }
    }

    // 1.2
    fn walk(commands: &str) -> (i32, i32) {
        let (mut x, mut y) = (0, 0);
        let mut dir = Direction::North;
        for c in commands.chars() {
            match c {
                'L' => dir = dir.turn_left(),
                'R' => dir = dir.turn_right(),
                'F' => match dir {
                    Direction::North => y += 1,
                    Direction::East => x += 1,
                    Direction::South => y -= 1,
                    Direction::West => x -= 1,
                },
                _ => {}
            }
        }
        (x, y)
    }

    println!("{:?}", walk("FFRFF")); // (2, 2)
    println!("{:?}", walk("LLFF?")); // (0, -2)

    // 1.3: with a variant `Up`, both `match self` in turn_right and `match dir`
    // in walk fail with "non-exhaustive patterns: `Direction::Up` not covered".
    // The compiler lists every place that must handle the new variant, instead
    // of the program silently doing the wrong thing at runtime.
}

/// Task 2 — Enums with data
fn task2() {
    enum Shape {
        Circle { radius: f64 },
        Rectangle { width: f64, height: f64 },
    }

    impl Shape {
        // 2.1
        fn area(&self) -> f64 {
            match self {
                Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
                Shape::Rectangle { width, height } => width * height,
            }
        }

        // 2.2: the arms are tried from top to bottom, so the special cases
        // (a point, a square) must come before the general ones.
        fn describe(&self) -> String {
            match self {
                Shape::Circle { radius: 0.0 } => "a point".to_string(),
                Shape::Rectangle { width, height } if width == height => {
                    format!("a square of side {width}")
                }
                Shape::Circle { .. } => "a circle".to_string(),
                Shape::Rectangle { .. } => "a rectangle".to_string(),
            }
        }
    }

    let shapes = [
        Shape::Circle { radius: 0.0 },
        Shape::Circle { radius: 1.0 },
        Shape::Rectangle {
            width: 2.0,
            height: 2.0,
        },
        Shape::Rectangle {
            width: 2.0,
            height: 3.0,
        },
    ];
    for s in &shapes {
        println!("{}: area {:.2}", s.describe(), s.area());
    }
    // a point: area 0.00
    // a circle: area 3.14
    // a square of side 2: area 4.00
    // a rectangle: area 6.00

    // 2.3: with the enum, a new OPERATION is one new function with a match,
    // but a new SHAPE means editing every existing match. With classes it is
    // the other way round: a new shape is one new class, but a new operation
    // means adding a method to every class.
}

/// Task 3 — `Option`
fn task3() {
    // 3.1
    fn find_index(values: &[i32], target: i32) -> Option<usize> {
        for (i, &v) in values.iter().enumerate() {
            if v == target {
                return Some(i);
            }
        }
        None
    }

    println!("{:?}", find_index(&[5, 6, 7], 7)); // Some(2)
    println!("{:?}", find_index(&[5, 6, 7], 9)); // None

    // 3.2: each `?` returns `None` from the function if that step fails.
    fn parse_point(s: &str) -> Option<(i32, i32)> {
        let (x, y) = s.split_once(',')?;
        Some((x.trim().parse().ok()?, y.trim().parse().ok()?))
    }

    println!("{:?}", parse_point(" -1 , 7 ")); // Some((-1, 7))
    println!("{:?}", parse_point("3;4")); // None
    println!("{:?}", parse_point("3,x")); // None

    // 3.3
    for input in ["3,4", "3;4"] {
        if let Some((x, y)) = parse_point(input) {
            println!("x = {x}, y = {y}");
        } else {
            println!("invalid");
        }
    }

    // `let ... else` suits a function that should stop early when the value is
    // missing, so the rest of it can use `x` and `y` directly.
    fn print_point(input: &str) {
        let Some((x, y)) = parse_point(input) else {
            println!("invalid");
            return;
        };
        println!("x = {x}, y = {y}");
    }

    print_point("3,4"); // x = 3, y = 4
    print_point("3;4"); // invalid

    // 3.4
    fn double_first(v: &[i32]) -> Option<i32> {
        v.first().map(|n| n * 2)
    }

    // `chars().count()` counts letters; `len()` would count bytes ("Łódź" is 7 bytes).
    fn name_length(name: Option<&str>) -> usize {
        name.map_or(0, |n| n.chars().count())
    }

    println!("{:?}", double_first(&[21])); // Some(42)
    println!("{:?}", double_first(&[])); // None
    println!("{}", name_length(Some("Łódź"))); // 4
    println!("{}", name_length(None)); // 0

    // 3.5: `unwrap` is acceptable in tests, quick experiments, and where `None`
    // really cannot happen (then it would be a bug in our own code).
    // `expect("reason")` panics the same way, but the message says why we
    // expected a value, which makes the crash much easier to understand.
}

/// Task 5 — A tour of patterns
fn task5() {
    // 5.1
    fn quadrant(p: (i32, i32)) -> &'static str {
        match p {
            (0, 0) => "origin",
            (_, 0) => "on x axis",
            (0, _) => "on y axis",
            _ => "elsewhere",
        }
    }

    println!("{}", quadrant((0, 0))); // origin
    println!("{}", quadrant((3, 0))); // on x axis
    println!("{}", quadrant((0, 5))); // on y axis
    println!("{}", quadrant((1, 2))); // elsewhere

    // 5.2: `n @ pattern` checks the pattern and also names the matched value.
    fn age_group(age: u32) -> String {
        match age {
            n @ 0..=12 => format!("child ({n})"),
            n @ 13..=19 => format!("teen ({n})"),
            n => format!("adult ({n})"),
        }
    }

    println!("{}", age_group(7)); // child (7)
    println!("{}", age_group(15)); // teen (15)
    println!("{}", age_group(40)); // adult (40)

    // 5.3: `..` matches any number of elements in the middle.
    fn describe_list(v: &[i32]) -> String {
        match v {
            [] => "empty".to_string(),
            [x] => format!("one: {x}"),
            [first, .., last] => format!("from {first} to {last}"),
        }
    }

    println!("{}", describe_list(&[])); // empty
    println!("{}", describe_list(&[4])); // one: 4
    println!("{}", describe_list(&[1, 2, 3])); // from 1 to 3
}
