#[test]
enum Direction {
    East,
    West,
    North,
    South,
}

fn test1() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => { // Matching South or North here
            println!("South or North");
        },
        _ => println!("Other direction"),
    };
}

#[test]
fn test2() {
    let boolean = true;

    // Using match to assign binary value based on boolean
    let binary = match boolean {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);

    println!("Success!");
}

#[test]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn test3() {
    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg);
    }

    println!("Success!");
}

fn show_message(msg: Message) {
    match msg {
        Message::Move { x: a, y: b } => { // match Message::Move
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        },
        _ => println!("no data in these variants"),
    }
}

#[test]
fn test4() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    // Using `matches!` to check if each character is an uppercase letter
    for ab in alphabets {
        if matches!(ab, 'A'..='Z') {
            println!("{} is uppercase", ab);
        } else {
            println!("{} is not uppercase", ab); // Handle non-uppercase letters
        }
    }

    println!("Success!");
}

#[test]
#[derive(PartialEq)]  // Deriving the PartialEq trait to enable comparison
enum MyEnum {
    Foo,
    Bar,
}

fn test5() {
    let mut count = 0;

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        if e == MyEnum::Foo { // Now this works because PartialEq is derived
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success!");
}

#[test]
fn test6() {
    let o = Some(7);

    // Using `if let` to simplify the match expression
    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
        println!("Success!");
    }
}

#[test]
enum Foo {
    Bar(u8),
}

fn test7() {
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);
        println!("Success!");
    }
}

#[test]
enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn test8() {
    let a = Foo::Qux(10);

    // Using `match` to handle all variants
    match a {
        Foo::Bar => println!("match foo::bar"),
        Foo::Baz => println!("match foo::baz"),
        _ => println!("match others"),
    }
}

#[test]
fn test9() {
    let age = Some(30);

    // Shadow the `age` variable with the unwrapped value inside the `Some` variant
    if let Some(age) = age {
        assert_eq!(age, 30);  // Now `age` is the value inside the `Some` variant, so we just check the value 30
    } // The new variable `age` goes out of scope here

    // Now, `age` is back to being the original `Some(30)` value
    match age {
        Some(age) => println!("age is a new variable, its value is {}", age),
        _ => (),
    }
}

//////////////////

#[test]
fn test10() {}

fn match_number(n: i32) {
    match n {
        // Match a single value
        1 => println!("One!"),
        // Match 2 to 5 using `|` for multiple values
        2 | 3 | 4 | 5 => println!("match 2 -> 5"),
        // Match an inclusive range 6 to 10
        6..=10 => {
            println!("match 6 -> 10")
        },
        // Catch-all for other values
        _ => {
            println!("match -infinite -> 0 or 11 -> +infinite")
        }
    }
}

#[test]
struct Point {
    x: i32,
    y: i32,
}

fn test11() {
    // Fill in the blank to let p match the second arm
    let p = Point { x: 3, y: 20 };  // This will match the second arm because x is between 0 and 5, and y is one of 10, 20, or 30.

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // Second arm, matching x in range 0..=5 and y being 10, 20, or 30
        Point { x: 0..=5, y: y@(10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

#[test]
enum Message {
    Hello { id: i32 },
}

fn test12() {
    let msg = Message::Hello { id: 5 };

    match msg {
        // First arm: Matches id in range [3, 7] and binds it to `id`
        Message::Hello { id } if id >= 3 && id <= 7 => println!("Found an id in range [3, 7]: {}", id),

        // Second arm: Matches id being 10, 11, or 12 and binds it to `newid`
        Message::Hello { id: newid @ (10 | 11 | 12) } => {
            println!("Found an id in another range [10, 12]: {}", newid)
        }

        // Default arm: Matches any other id
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

#[test]
fn test13() {
    let num = Some(4);
    let split = 5;

    match num {
        // Match if the value of x is less than split (match guard)
        Some(x) if x < split => assert!(x < split),

        // Match if the value of x is greater than or equal to split
        Some(x) => assert!(x >= split),

        // Match None case
        None => (),
    }

    println!("Success!");
}

#[test]
fn test14() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .., last) => {
            assert_eq!(first, 2);  // First element is 2
            assert_eq!(last, 2048); // Last element is 2048
        }
    }

    println!("Success!");
}

#[test]
fn test15() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        value => value.push_str(" world!") // Here, `value` is a mutable reference to `v`
    }

    println!("{}", v); // Prints "hello, world!"
}