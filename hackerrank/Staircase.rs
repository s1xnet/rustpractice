use std::io::{self, BufRead};

/*
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn staircase(n: i32) {
    // Loop through each row
    for i in 1..=n {
        // Print n - i spaces
        for _ in 0..(n - i) {
            print!(" ");
        }
        // Print i # symbols
        for _ in 0..i {
            print!("#");
        }
        println!();
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Read the input integer n
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}