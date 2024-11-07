use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
// Function to compute GCD (Greatest Common Divisor)
fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcm(x: i32, y: i32) -> i32 {
    (x / gcd(x, y)) * y
}

fn lcm_of_array(arr: &[i32]) -> i32 {
    arr.iter().cloned().reduce(|acc, x| lcm(acc, x)).unwrap_or(1)
}

fn gcd_of_array(arr: &[i32]) -> i32 {
    arr.iter().cloned().reduce(|acc, x| gcd(acc, x)).unwrap_or(1)
}

fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    // Find the LCM of the first array
    let lcm_a = lcm_of_array(a);

    // Find the GCD of the second array
    let gcd_b = gcd_of_array(b);

    // Count how many multiples of lcm_a divide gcd_b
    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let total = getTotalX(&arr, &brr);

    writeln!(&mut fptr, "{}", total).ok();
}