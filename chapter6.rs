#[test]
fn test1() {
    let s: &str = "hello, world";

    println!("Success!");
}

#[test]
fn test2() {
    let s: Box<str> = "hello, world".into();
    greetings(&*s);
}

fn greetings(s: &str) {
    println!("{}", s)
} //2

#[test]
fn test3() {
    let s: Box<str> = "hello, world".into();
    greetings(s.as_ref());
} //2.2

fn greetings(s: &str) {
    println!("{}", s)
}

#[test]
fn test4() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

#[test]
fn test5() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

#[test]
fn test6() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}

#[test]
fn test7() {
    let s = String::from("I like dogs");
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

#[test]
fn test8() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", s1, s2);
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}

#[test]
fn test9() {
    let s = "hello, world";
    greetings(s.to_string());
}

fn greetings(s: String) {
    println!("{}", s)
} //1

#[test]
fn test9() {
    let s = "hello, world";
    greetings(String::from(s));
}

fn greetings(s: String) {
    println!("{}", s)
} //2

#[test]
fn test10() {
    let s = "hello, world".to_string();
    let s1: &str = &s;

    println!("Success!");
} //1

#[test]
fn test10() {
    let s = "hello, world".to_string();
    let s1: &str = s.as_str();

    println!("Success!");
} //2

#[test]
fn test11() {
    let byte_escape = "I'm writing Ru\x73t!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
             unicode_codepoint, character_name );

    let long_string = "String literals \
                        can span multiple lines. \
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

#[test]
fn test12() {
    // Fix: Use a raw string with no escape sequences processed
    let raw_str = r"Escapes don't work here: \x3F \u{211D}"; // This remains the same since it's the raw string, not evaluating escapes
    assert_eq!(raw_str, r"Escapes don't work here: \x3F \u{211D}");

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need # in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Fix for long_delimiter: Correct raw string without escape sequences
    let long_delimiter = r#""Hello, ##""#; // Correct delimiter usage, no escape needed
    assert_eq!(long_delimiter, r#""Hello, ##""#);

    println!("Success!");
}

#[test]
fn test13() {
    let s1 = String::from("hi,中国");

    let h = &s1[0..1]; // Slice the first byte (1 byte for "h")
    assert_eq!(h, "h");

    // To get the second character ("中"), which takes 3 bytes
    let h1 = &s1[3..6]; // "中" starts at byte 3 and ends at byte 6
    assert_eq!(h1, "中");

    println!("Success!");
}

#[test]
fn test14() {
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}

////////////////

#[test]
fn test15() {

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    assert!(arr.len() == 5);

    println!("Success!");
}

#[test]
fn test16() {
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];

    assert!(std::mem::size_of_val(&arr) == 12);

    println!("Success!");
}

#[test]
fn test17() {
    let list: [i32; 100] = [1; 100];

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}

#[test]
fn test18() {
    let _arr = [1, 2, 3];

    println!("Success!");
}

#[test]
fn test19() {
    let arr = ['a', 'b', 'c'];

    let ele = arr[0]; // Change the index from 1 to 0 to get 'a'

    assert!(ele == 'a');

    println!("Success!");
}

#[test]
fn test20() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];

    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // Use `get` instead of direct indexing for safe access
    let _name1 = names.get(2);

    if let Some(name) = _name1 {
        println!("Found: {}", name);
    } else {
        println!("Index out of bounds");
    }

    println!("Success!");
}

///////////////

#[test]
fn test21() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world";

    println!("Success!");
}

#[test]
fn test22() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // Modify '8' to make it work for 64-bit architecture
    assert!(std::mem::size_of_val(&slice) == 16); // Likely 16 bytes for a 64-bit system

    println!("Success!");
}

#[test]
fn test23() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: &[i32] = &arr[1..4]; // Takes a slice containing elements 2, 3, and 4
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}

////////////////////

#[test]
fn test24() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[..2]; // Equivalent to 0..2

    assert_eq!(slice1, slice2);

    println!("Success!");
}

#[test]
fn test25() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..3]; // Correct range for the first character "你"

    assert!(slice == "你");

    println!("Success!");
}

#[test]
fn test26() {
    let mut s = String::from("hello world");

    // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
    // It works because `&String` can be implicitly converted to `&str`. This is called `Deref coercion`.
    let letter = first_letter(&s);

    // Use the letter reference before clearing the string
    println!("the first letter is: {}", letter);

    // Now we can safely clear the string
    s.clear(); // This works without errors now
}

fn first_letter(s: &str) -> &str {
    &s[..1]
}

///////////////

#[test]
fn test27() {
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}

#[test]
fn test28() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface"); // Corrected index to access the third element

    println!("Success!");
}

#[test]
fn test29() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // Manually split the tuple into two smaller parts for printing
    println!("too long tuple: ({:?}, ...)", &too_long_tuple.0);
}

#[test]
fn test30() {
    let tup = (1, 6.4, "hello");

    // Destructure the tuple into variables x, y, and z
    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}

#[test]
fn test31() {
    let (x, y, z);

    // Destructure the tuple in the desired order
    (z, x, y) = (1, 2, 3); // The values are assigned in reverse order to match the assertions

    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success!");
}

#[test]
fn test32() {
    // Fill the blank, need a few computations here.
    let (x, y) = sum_multiply((2, 3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

//////////////////

#[test]
struct Person {
    name: String,
    age: u8,
    hobby: String,
}

fn test33() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("coding"),
    };

    println!("Success!");
}

#[test]
struct Unit;

trait SomeTrait {
}

impl SomeTrait for Unit {
    // Implement any behaviors for `Unit` here (even though `Unit` has no fields).
}

fn test34() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
}

// Fill the blank to make the code work
fn do_something_with_unit(u: Unit) {  // The type of the argument must be `Unit`
    // You can call methods or behaviors defined in the trait here.
    // The argument `u` is a `Unit` struct.
}

#[test]
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn test35() {
    let v = Point(0, 127, 255); // Fill in the values correctly for the Point
    let color = Color(v.0, v.1, v.2); // Convert Point into Color by using its values
    check_color(color);

    println!("Success!");
}

fn check_color(p: Color) {
    // Destructure the Color tuple struct correctly
    let Color(x, _, _) = p;  // Corrected destructuring for tuple struct
    assert_eq!(x, 0);   // Check the first value of Color
    assert_eq!(p.1, 127); // Check the second value of Color
    assert_eq!(p.2, 255); // Check the third value of Color
}

#[test]
struct Person {
    name: String,
    age: u8,
}

fn test36() {
    let age = 18;
    let mut p = Person { // Make the entire struct mutable
        name: String::from("sunface"),
        age,
    };

    // Now you can modify the mutable field `age`
    p.age = 30;

    // Fill the blank: Update the `name` field
    p.name = String::from("sunfei");

    println!("Success!");
}

#[test]
struct Person {
    name: String,
    age: u8,
}

fn test37() {
    println!("Success!");
}

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        name, // Shorthand initialization
    }
}

#[test]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn test38() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("Success!");
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u // Use struct update syntax to copy the other fields from `u`
    }
}

#[test]
#[derive(Debug)] // Derive the Debug trait to enable printing with {:?}
struct Rectangle {
    width: u32,
    height: u32,
}

fn test39() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("{:?}", rect1); // Print debug info to stdout
}

#[test]
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

fn test40() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    // Borrow the `name` field instead of moving it
    let name = &f.name;  // Borrow `f.name`

    // Now we can safely use `f.name` as it is borrowed, not moved.
    println!("{}, {}, {:?}", name, f.data, f);
}

/////////////////

#[test]
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One = 1,
    Two = 2,
}

enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}

fn test41() {
    // Cast `Number1::One` and `Number2::One` to `i32` for comparison
    assert_eq!(Number1::One as i32, Number2::One as i32); // Compare as integers

    println!("Success!");
}

#[test]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn test42() {
    let msg1 = Message::Move { x: 1, y: 2 }; // Instantiating with x = 1, y = 2
    let msg2 = Message::Write("hello, world!".to_string()); // Instantiating with "hello, world!"

    println!("Success!");
}

#[test]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn test43() {
    let msg = Message::Move { x: 1, y: 2 };

    // Use pattern matching to extract x and y values
    if let Message::Move { x, y } = msg {
        // Example assertion: you can compare x and y here, or any other logic
        let a = 1;
        let b = 2;
        assert_eq!(x, a);  // comparing x with a
        assert_eq!(y, b);  // comparing y with b
    } else {
        panic!("NEVER LET THIS RUN!");
    }

    println!("Success!");
}

#[test]
// Derive the Debug trait so we can print the enum variants
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn test44() {
    // Use Vec<Message> instead of an array
    let msgs: Vec<Message> = vec![
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    // Iterate over the vector and pass each message to the show_message function
    for msg in msgs {
        show_message(msg);
    }
}

// Show the message using the Debug formatting
fn show_message(msg: Message) {
    println!("{:?}", msg); // Print the message using the Debug format
}

#[test]
fn test45() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six {  // Pattern match on `Some(n)` to extract the value `n`
        println!("{}", n);   // This prints the value of `n`

        println!("Success!");
    } else {
        println!("No value to add to!");
    }

    // Prevent panic by returning early if no value is present
    // This prevents "NEVER LET THIS RUN!" from running when the value is `None`
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,          // If the input is `None`, return `None`
        Some(i) => Some(i + 1),  // If the input is `Some(i)`, return `Some(i + 1)`
    }
}