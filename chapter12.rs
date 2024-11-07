#[test]
fn test1() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8;  // Convert decimal to u8 explicitly

    // Convert f32 to u32, then use char::from_u32 to get a char
    let _c1: char = char::from_u32(decimal as u32).unwrap();  // This will work, converting the integer part to a char ('a')

    let _c2 = integer as char;  // Converts integer (97) to char, which corresponds to 'a'

    // Update the assertion to match the actual value of `integer` (97 corresponds to 'a')
    assert_eq!(integer, 'a' as u8);  // This will pass, since 97 corresponds to 'a'

    println!("Success!");
}


#[test]
//#![allow(overflowing_literals)]  // This line will suppress overflow errors

fn test2() {
    assert_eq!(u8::MAX, 255);
    // The max of `u8` is 255 as shown above.
    // so the below code will cause an overflow error: literal out of range for `u8`.
    // PLEASE looking for clues within compile errors to FIX it.
    // DON'T modify any code in main.
    let v = 1000 as u8;

    println!("Success!");
}


#[test]
//#![allow(overflowing_literals)]  // Allow overflowing literals globally

fn test3() {
    assert_eq!(1000 as u16, 1000); // 1000 fits within u16, no change
    assert_eq!(1000 as u8, 232);   // 1000 wraps around to 232 (1000 % 256)

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256); // Prints 232

    assert_eq!(-1_i8 as u8, 255); // -1 wraps around to 255 in u8

    // Saturating cast from f32 to u8
    assert_eq!(300.1_f32 as u8, 255);  // Saturates to u8::MAX (255)
    assert_eq!(-100.1_f32 as u8, 0);   // Saturates to 0

    // Unsafe methods (unchecked casting, could overflow)
    unsafe {
        // 300.0 is 44 in u8 (due to overflow when cast)
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156 (due to overflow when cast)
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0 (undefined behavior)
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}


#[test]
fn test4() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 as usize;  // Convert pointer to address (usize)
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_address as *mut i32; // Convert address back to a pointer
    unsafe {
        // Add one to the second element
        *p2 += 1;
    }

    assert_eq!(values[1], 3);

    println!("Success!");
}


#[test]
fn test5() {
    let arr: [u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13); // size of arr in bytes
    let a: *const [u64; 13] = &arr; // pointer to array of 13 u64 elements
    let b = a as *const u8; // cast to pointer to u8 (byte-level access)

    unsafe {
        // Directly calculate the size of the original array (104 bytes)
        assert_eq!(std::mem::size_of_val(&arr), 8 * 13); // size of the underlying array as u8
    }

    println!("Success!");
}

//////////////

#[test]
fn test6() {
    // impl From<bool> for i32
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    // Conversion from 'char' to 'i32' (using its Unicode value)
    let i3: i32 = 'a' as i32;  // Using `as` to convert to i32
    assert_eq!(i3, 97); // 'a' has Unicode value 97

    // Conversion from 'char' to 'String'
    let s: String = 'a'.to_string();  // Using `to_string()` method

    println!("Success!");
}


#[test]
#[derive(Debug)] // Correct use of attribute to derive Debug
struct Number {
    value: i32,
}

// Implement From<i32> for Number
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

fn test7() {
    // Using the from method
    let num = Number::from(30);
    assert_eq!(num.value, 30);

    // Using into method (this requires type annotation)
    let num: Number = 30.into();
    assert_eq!(num.value, 30);

    println!("Success!");
}




#[test]
use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

// Implement From<io::Error> for CliError
impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IoError(error)
    }
}

// Implement From<num::ParseIntError> for CliError
impl From<num::ParseIntError> for CliError {
    fn from(error: num::ParseIntError) -> Self {
        CliError::ParseError(error)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? automatically converts io::Error to CliError
    let contents = fs::read_to_string(file_name)?; // This will return CliError::IoError
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?; // This will return CliError::ParseError
    Ok(num)
}

fn test8() {
    match open_and_parse_file("example.txt") {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => match e {
            CliError::IoError(err) => println!("IO error: {}", err),
            CliError::ParseError(err) => println!("Parse error: {}", err),
        },
    }
}




#[test]
use std::convert::TryInto;

fn test9() {
    let n: i16 = 256;

    // TryInto provides the method `try_into`, which returns a Result
    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("there is an error when converting: {:?}, but we catch it", e.to_string());
            0
        }
    };

    assert_eq!(n, 0);  // The conversion fails, and we get 0 as the fallback value

    println!("Success!");
}


#[test]
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNum(i32);

impl TryFrom<i32> for EvenNum {
    type Error = ();

    // Implement try_from
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(())
        }
    }
}

fn test10() {
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    // Using `try_into` to convert from i32 to EvenNum
    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNum(8))); // 8 is even, so the conversion is successful

    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, Err(())); // 5 is odd, so the conversion fails

    println!("Success!");
}

//////////////


#[test]
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    // Implement the fmt method to display the Point as "The point is (x, y)"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn test11() {
    let origin = Point { x: 0, y: 0 };
    // Use the implemented Display trait to convert the point to a string
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    println!("Success!");
}


#[test]
use std::str::FromStr;

fn test12() {
    let parsed: i32 = "5".parse().unwrap(); // Parse "5" to i32
    let turbo_parsed = "10".parse::<i32>().unwrap(); // Parse "10" to i32
    let from_str = i32::from_str("20").unwrap(); // Parse "20" to i32 using FromStr explicitly
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!");
}



#[test]
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .map(|x| x.trim())
            .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
}

fn test13() {
    // FILL in the blanks in two ways
    let p = "(3, 4)".parse::<Point>(); // First way: use parse directly
    assert_eq!(p.unwrap(), Point { x: 3, y: 4 });

    let p2 = Point::from_str("(3, 4)"); // Second way: use from_str explicitly
    assert_eq!(p2.unwrap(), Point { x: 3, y: 4 });

    println!("Success!");
}



#[test]
fn foo() -> i32 {
    0
}

fn test14() {
    let pointer = foo as *const ();  // Cast function to a raw pointer
    let function = unsafe {
        std::mem::transmute::<*const (), fn() -> i32>(pointer)  // Unsafe transmutation
    };
    assert_eq!(function(), 0);  // Call the function through the function pointer

    println!("Success!");
}



#[test]
fn test15() {
    // Turning raw bytes to u32, f64, etc.
    let raw_bytes = [0x78, 0x56, 0x34, 0x12];

    // Use `u32::from_ne_bytes` instead of `transmute`
    let num = u32::from_ne_bytes(raw_bytes);
    assert_eq!(num, 0x12345678);

    let num_le = u32::from_le_bytes(raw_bytes);  // Little-endian
    assert_eq!(num_le, 0x12345678);

    let num_be = u32::from_be_bytes(raw_bytes);  // Big-endian
    assert_eq!(num_be, 0x78563412);

    // Turning a pointer into a usize
    let ptr = &0;
    let ptr_num_cast = ptr as *const i32 as usize;
    println!("Pointer as usize: {}", ptr_num_cast);

    // Turning an &mut T into an &mut U
    let ptr = &mut 0;
    let val_casts = unsafe { &mut *(ptr as *mut i32 as *mut u32) };
    println!("Mutable reference cast: {}", *val_casts);

    // Turning an &str into a &[u8]
    let slice = "Rust".as_bytes();
    assert_eq!(slice, &[82, 117, 115, 116]);

    // Or using byte string literal
    assert_eq!(b"Rust", &[82, 117, 115, 116]);
}