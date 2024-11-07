#[test]
fn test1() {
    // The assertion should work correctly, no panic.
    assert_eq!("abc".as_bytes(), &[97, 98, 99]);

    let v = vec![1, 2, 3];

    // Fix index out of bounds by safely accessing the element with get()
    if let Some(ele) = v.get(3) {
        println!("Element at index 3: {}", ele);
    } else {
        println!("Index 3 is out of bounds for the vector.");
    }

    // Unwrapping safely by checking if there's a value
    let ele = v.get(3).unwrap_or(&0);  // Use `unwrap_or` to provide a fallback value instead of panicking
    println!("Element at index 3 (or fallback): {}", ele);

    // Fixing overflow by adding a valid range check for speed
    let v = production_rate_per_hour(2);
    println!("Production rate per hour: {}", v);

    // Safely handling division by zero
    divide(15, 0);

    println!("Success!")
}

fn divide(x: u8, y: u8) {
    if y == 0 {
        println!("Error: Division by zero!");
    } else {
        println!("{}", x / y);
    }
}

fn production_rate_per_hour(speed: u8) -> f64 {
    let cph: u8 = 221;

    // Cast speed and cph to u16 to prevent overflow
    let speed_u16 = speed as u16;
    let cph_u16 = cph as u16;

    match speed {
        1..=4 => (speed_u16 * cph_u16) as f64,
        5..=8 => (speed_u16 * cph_u16) as f64 * 0.9,
        9..=10 => (speed_u16 * cph_u16) as f64 * 0.77,
        _ => 0.0, // Return a valid number instead of 0 as f64
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}

/////////////

#[test]
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();

    // Return early if parsing fails
    match (n1, n2) {
        (Ok(n1), Ok(n2)) => Ok(n1 * n2),  // Both parsed successfully
        (Err(e), _) | (_, Err(e)) => Err(e), // One of the parses failed
    }
}

fn test2() {
    let result = multiply("10", "2");
    assert_eq!(result, Ok(20));  // Check if result is Ok(20)

    let result = multiply("t", "2");
    assert_eq!(result.is_err(), true);  // Check if result is an Err, meaning failure

    println!("Success!");
}


#[test]
use std::num::ParseIntError;

// Implement multiply with `?`
fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>()?;  // Use `?` to return early if parsing fails
    let n2 = n2_str.parse::<i32>()?;  // Use `?` to return early if parsing fails

    Ok(n1 * n2)  // If both parse successfully, return the multiplication result
}

fn test3() {
    assert_eq!(multiply("3", "4").unwrap(), 12);  // Expect 12 after parsing and multiplying
    println!("Success!");  // If no panics or errors, print Success!
}


#[test]
use std::fs::File;
use std::io::{self, Read};

fn read_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// FILL in the blanks with one code line
// DON'T change any code lines
fn read_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;  // Use ? to handle errors concisely

    Ok(s)
}

fn test4() {
    assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
    println!("Success!");
}


#[test]
use std::num::ParseIntError;

// FILL in the blank in two ways: map, and then
fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
    // Using map
    n_str.parse::<i32>().map(|n| n + 2)

    // Or using and_then
    // n_str.parse::<i32>().and_then(|n| Ok(n + 2))
}

fn test5() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!");
}



#[test]
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    n1_str.parse::<i32>()
        .and_then(|n1| n2_str.parse::<i32>().map(|n2| n1 * n2))  // `and_then` and `map` combination
}

fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    n1_str.parse::<i32>()
        .and_then(|n1| n2_str.parse::<i32>().map(|n2| n1 * n2))  // Same as multiply
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn test6() {
    // This still presents a reasonable answer.
    let twenty = multiply1("10", "2");
    print(twenty);

    // The following now provides a much more helpful error message.
    let tt = multiply("t", "2");  // Now calling the multiply function
    print(tt);

    println!("Success!");
}



#[test]
use std::num::ParseIntError;

// Define a type alias for Result<i32, ParseIntError>
type Res<T> = Result<T, ParseIntError>;

// Use the above alias to refer to our specific `Result` type.
fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// Here, the alias again allows us to save some space.
fn print(result: Res<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn test7() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));

    println!("Success!");
}


#[test]
use std::num::ParseIntError;

fn test8() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),  // Return early with an error if parsing fails
    };

    println!("{}", number);  // This will only print if the parsing is successful

    Ok(())  // Return Ok if everything succeeds
}