#[test]
// Fill in the blanks to make it work
struct A;          // Concrete type `A`.
struct S(A);       // Concrete type `S` with `A` as a member.
struct SGen<T>(T); // Generic type `SGen` with a type parameter `T`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn test1() {
    // Using the non-generic functions
    reg_fn(S(A));          // Concrete type.
    gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(42)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('c'));

    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen('c'));

    println!("Success!");
}

#[test]
use std::ops::Add;

// Implement the generic function below.
fn sum<T>(a: T, b: T) -> T
where
    T: Add<Output = T>, // T must implement the Add trait, and the result of the addition should be of type T
{
    a + b
}

fn test2() {
    assert_eq!(5, sum(2i8, 3i8));      // i8 example
    assert_eq!(50, sum(20, 30));       // i32 example
    assert_eq!(2.46, sum(1.23, 1.23)); // f64 example

    println!("Success!");
}

#[test]
// Define the generic struct Point.
struct Point<T> {
    x: T,
    y: T,
}

fn test3() {
    // Create instances of Point with integer and floating-point types.
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("Success!");
}

#[test]
// Define the struct Point with two generic types for x and y.
struct Point<T, U> {
    x: T,
    y: U,
}

fn test4() {
    // The `x` is an integer (5), and `y` is a string ("hello").
    let p = Point { x: 5, y: "hello".to_string() };

    println!("Success!");
}

#[test]
// Add generic for Val to make the code work
struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    // Generic method `value` that returns a reference to the value of type T
    fn value(&self) -> &T {
        &self.val
    }
}

fn test5() {
    let x = Val { val: 3.0 };           // `x` is a Val<f64>
    let y = Val { val: "hello".to_string() }; // `y` is a Val<String>

    // We can print the values using `value()` method for both `x` and `y`
    println!("{}, {}", x.value(), y.value());
}

//////////////////

#[test]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // Implement the `mixup` method to take another `Point` and return a new `Point`
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn test6() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中' };

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success!");
}

#[test]
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn test7() {
    let p = Point { x: 5.0, y: 10.0 }; // Change the values to `f32` by using `5.0` and `10.0`
    println!("{}", p.distance_from_origin());
}

///////////////

#[test]
struct Array<T, const N: usize> {
    data: [T; N],
}

fn test8() {
    let arrays = [
        Array {
            data: [1, 2, 3], // Array<i32, 3>
        },
        Array {
            data: [4, 5, 6], // Array<i32, 3>
        },
        // This will fail if you try to add:
        // Array {
        //     data: [1, 2], // Array<i32, 2> (different type)
        // }
    ];

    println!("Success!");
}

#[test]
// Fill in the blanks to make it work.
fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

fn test9() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}

/////////////////

#[test]
trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;
}

struct Student {}
impl Hello for Student {
    fn say_something(&self) -> String {
        String::from("I'm a good student")
    }
}

struct Teacher {}
impl Hello for Teacher {
    fn say_hi(&self) -> String {
        String::from("Hi, I'm your new teacher")
    }

    fn say_something(&self) -> String {
        String::from("I'm not a bad teacher")
    }
}

fn test10() {
    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("Success!");
}

#[test]
// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

// ADD some attributes to make the code work!
#[derive(Debug, PartialEq, PartialOrd)]  // Deriving the traits for comparison and printing
struct Seconds(i32);

fn test11() {
    let _one_second = Seconds(1);

    println!("One second looks like: {:?}", _one_second);
    let _this_is_true = (_one_second == _one_second);  // Compares Seconds using PartialEq
    let _this_is_false = (_one_second > _one_second);  // Compares Seconds using PartialOrd

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}

#[test]
use std::ops::Mul;

// Implement fn multiply to make the code work.
fn multiply<T>(a: T, b: T) -> T
where
    T: Mul<Output = T>,  // T must implement the Mul trait
{
    a * b  // Uses the * operator, which calls the Mul trait's `mul` method
}

fn test12() {
    assert_eq!(6, multiply(2u8, 3u8));   // Multiplies two u8 values
    assert_eq!(5.0, multiply(1.0, 5.0)); // Multiplies two f64 values

    println!("Success!");
}

#[test]
trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}

// Implement `fn summary` to accept references instead of owning the values
fn summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

fn test13() {
    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };

    summary(&post);
    summary(&weibo);

    println!("{:?}", post);
    println!("{:?}", weibo);
}

#[test]
struct Sheep;
struct Cow;

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}

// Using a trait object (Box<dyn Animal>) to return different types.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep)
    } else {
        Box::new(Cow)
    }
}

fn test14() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}

#[test]
use std::ops::Add;

fn sum<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

fn test15() {
    assert_eq!(sum(1, 2), 3); // This will work for integers
    println!("Success!");
}

////////////////////

#[test]
use std::fmt;

// Derive PartialEq so we can use PartialOrd
#[derive(PartialEq)]
struct Unit(i32);

// Implement Debug for Unit so we can use {:?}
impl fmt::Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unit({})", self.0)
    }
}

// Implement PartialOrd for Unit to allow comparisons
impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}

fn test16() {
    let pair = Pair {
        x: Unit(1),
        y: Unit(3),
    };

    pair.cmp_display();
}

#[test]
trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming");
    }
}

struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the swan is flying");
    }
}

impl Bird for Duck {
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}

fn test17() {
    let duck = Duck; // FILL in the blank.
    duck.swim();

    let bird = hatch_a_bird(2);
    // This bird has forgotten how to swim, so below line will cause an error.
    // bird.swim();
    // But it can quack.
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    // This bird has forgotten how to fly, so below line will cause an error.
    // bird.fly();
    // But it can quack too.
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");
}

// IMPLEMENT this function.
fn hatch_a_bird(kind: i32) -> Box<dyn Bird> {
    if kind == 1 {
        Box::new(Swan)
    } else {
        Box::new(Duck)
    }
}

#[test]
trait Bird {
    fn quack(&self);
}

struct Duck;
impl Duck {
    fn fly(&self) {
        println!("Look, the duck is flying");
    }
}

struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the swan is flying");
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}

fn test18() {
    // FILL in the blank to make the code work.
    let birds: Vec<Box<dyn Bird>> = vec![
        Box::new(Duck), // A Duck wrapped in a Box<dyn Bird>
        Box::new(Swan), // A Swan wrapped in a Box<dyn Bird>
    ];

    for bird in birds {
        bird.quack();
        // When duck and swan turn into Birds, they all forgot how to fly, only remember how to quack.
        // So, the code below will cause an error.
        // bird.fly(); // This won't compile because `bird` is a `dyn Bird` and doesn't have a `fly` method.
    }
}

#[test]
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn test19() {
    let x = 1.1f64;
    let y = 8u8;

    // Draw x.
    draw_with_box(Box::new(x));  // `x` is a `f64`, so we box it into `Box<dyn Draw>`

    // Draw y.
    draw_with_ref(&y);  // `y` is a `u8`, so we pass a reference to it (`&y`)

    println!("Success!");
}

fn draw_with_box(x: Box<dyn Draw>) {
    println!("{}", x.draw());
}

fn draw_with_ref(x: &dyn Draw) {
    println!("{}", x.draw());
}

#[test]
trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

// Static dispatch using generics and trait bounds
fn static_dispatch<T: Foo>(item: T) {
    println!("{}", item.method());
}

// Dynamic dispatch using trait objects
fn dynamic_dispatch(item: &dyn Foo) {
    println!("{}", item.method());
}

fn test20() {
    let x = 5u8;
    let y = "Hello".to_string();

    // Static dispatch
    static_dispatch(x);

    // Dynamic dispatch
    dynamic_dispatch(&y);

    println!("Success!");
}

#[test]
trait MyTrait {
    fn f(&self) -> Box<dyn MyTrait>;
}

impl MyTrait for u32 {
    fn f(&self) -> Box<dyn MyTrait> {
        Box::new(42_u32)
    }
}

impl MyTrait for String {
    fn f(&self) -> Box<dyn MyTrait> {
        Box::new(self.clone())
    }
}

fn my_function(x: Box<dyn MyTrait>)  {
    x.f();
}

fn test21() {
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));

    println!("Success!");
}

////////////////

#[test]
use std::fmt;

struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, number_1: &A, number_2: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<A, B, C: Contains<A, B>>(container: &C) -> i32 {
    container.last() - container.first()
}

fn test22() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
             &number_1, &number_2,
             container.contains(&number_1, &number_2));

    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}

#[test]
use std::ops::Sub;

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}
impl Sub for Point<i32> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


fn test23() {
    assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
               Point { x: 1, y: 3 });

    println!("Success!");
}


#[test]
trait Pilot {
    fn fly(&self) -> String;
}

trait Wizard {
    fn fly(&self) -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) -> String {
        String::from("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("Up!")
    }
}

impl Human {
    fn fly(&self) -> String {
        String::from("*waving arms furiously*")
    }
}

fn test24() {
    let person = Human;

    assert_eq!(Pilot::fly(&person), "This is your captain speaking.");

    assert_eq!(Wizard::fly(&person), "Up!");

    assert_eq!(person.fly(), "*waving arms furiously*");

    println!("Success!");
}


#[test]
trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

struct CSStudent {
    name: String,
    university: String,
    fav_language: String,
    git_username: String,
}

impl Person for CSStudent {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Student for CSStudent {
    fn university(&self) -> String {
        self.university.clone()
    }
}

impl Programmer for CSStudent {
    fn fav_language(&self) -> String {
        self.fav_language.clone()
    }
}

impl CompSciStudent for CSStudent {
    fn git_username(&self) -> String {
        self.git_username.clone()
    }
}

fn test25() {
    let student = CSStudent {
        name: "Sunfei".to_string(),
        university: "XXX".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "sunface".to_string(),
    };

    println!("{}", comp_sci_student_greeting(&student));
}


#[test]
use std::fmt;

// DEFINE a newtype `Pretty` here
struct Pretty(String);

impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}

fn test26() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}