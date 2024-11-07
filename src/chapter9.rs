#[test]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Complete the area method which returns the area of the Rectangle.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn test1() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);

    println!("Success!");
}

#[test]
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(&self)  {  // Use `&self` to borrow the reference
        println!("the current state is {}", self.color);  // Access `color` through `self`
    }
}

fn test2() {
    let light = TrafficLight{
        color: "red".to_owned(),
    };
    // Don't take the ownership of `light` here.
    light.show_state();  // Call method with borrowed reference
    // ... Otherwise, there will be an error below
    println!("{:?}", light);  // `light` can still be used after the method call
}

#[test]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Using `&self` to borrow the reference to the instance
    pub fn show_state(&self) {
        println!("the current state is {}", self.color);
    }

    // Using `&mut self` to allow modifying the instance
    pub fn change_state(&mut self) {
        self.color = "green".to_string();
    }
}

fn test3() {
    println!("Success!");
}

#[test]
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // 1. Implement an associated function `new`,
    // 2. It will return a TrafficLight containing color "red"
    // 3. Must use `Self`, DON'T use `TrafficLight` in fn signatures or body
    pub fn new() -> Self {
        Self {
            color: "red".to_string(),
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

fn main() {
    let light = TrafficLight::new();  // Calling the associated function `new`
    assert_eq!(light.get_state(), "red");

    println!("Success!");
}

#[test]
struct Rectangle {
    width: u32,
    height: u32,
}

// First impl block with area method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Second impl block with can_hold method
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn test4() {
    println!("Success!");
}

#[test]
#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

impl TrafficLightColor {
    // Implement the color method that returns a string representation of the enum variant
    pub fn color(&self) -> &str {
        match self {
            TrafficLightColor::Red => "red",
            TrafficLightColor::Yellow => "yellow",
            TrafficLightColor::Green => "green",
        }
    }
}

fn test5() {
    let c = TrafficLightColor::Yellow;

    // Call the `color` method to get the string representation
    assert_eq!(c.color(), "yellow");

    println!("{:?}", c);
}