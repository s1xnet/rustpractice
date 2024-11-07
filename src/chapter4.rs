#[test]
fn test1() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;

    let z = 10;

    println!("Success!");
}

#[test]
fn test2() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}

#[test]
fn test3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}

#[test]
fn test5() {
    let v1 = 251_u8.wrapping_add(8);
    let v2 = i8::checked_add(251, 8).unwrap_or(0);

    println!("{},{}", v1, v2);
}

#[test]
fn test6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1_597);

    println!("Success!");
}

#[test]
fn test7() {
    let x = 1_000.000_1;
    let y: f32 = 0.12;
    let z = 0.01_f64;

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test8() {
    assert!((0.1 + 0.2 - 0.3).abs() < 1e-10);

    println!("Success!");
}

#[test]
fn test9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c as u8);
    }
}

#[test]
use std::ops::{Range, RangeInclusive};

fn test10() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

#[test]
fn test11() {

    assert!(1u32 + 2 == 3);

    assert!(1i32 - 2 == -1);
    assert!(1u8 - 2 == 255);

    assert!(3 * 50 == 150);

    assert!((9.6 / 3.2 - 3.0).abs() < 1e-10);

    assert!(24 % 5 == 4);

    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

#[test]
use std::mem::size_of_val;

fn test12() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");
}

#[test]
fn test13() {
    let c1 = "中";

    if let Some(c) = c1.chars().next() {
        print_char(c);
    }
}

fn print_char(c: char) {
    println!("{}", c);
}

#[test]
fn test14() {
    let _f: bool = false;

    let t = true;
    if !_f {
        println!("Success!");
    }
}

#[test]
fn test15() {
    let f = true;
    let t = true && true;
    assert_eq!(t, f);

    println!("Success!");
}

#[test]
fn test16() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, (2, 3));

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

#[test]
use std::mem::size_of_val;

fn test17() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}

#[test]
fn test18() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);

    println!("Success!");
}

#[test]
fn test19() {
    let v = {
        let x = 3;
        x
    };

    assert!(v == 3);

    println!("Success!");
}

#[test]
fn test20() {
    let s = sum(1, 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn test21() {
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn test22() {
    print();
}

fn print() -> () {
    println!("Success!");
}

#[test]
fn test23() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    loop {
    }
}

#[test]
fn test24() {
    let result = get_option(1);
    println!("{:?}", result);
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
        }
        _ => {

        }
    };

    never_return_fn()
}

fn never_return_fn() -> ! {
    loop {
    }
}

#[test]
fn test25() {

    let b = true;

    let _v = match b {
        true => 1,
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}