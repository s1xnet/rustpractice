use std::io::{self, BufRead};

/*
 * Complete the 'bonAppetit' function below.
 *
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY bill
 *  2. INTEGER k
 *  3. INTEGER b
 */

fn bonAppetit(bill: &[i32], k: i32, b: i32) {
    // Calculate the total cost of the items Anna and Brian ate (excluding the k-th item)
    let total_cost: i32 = bill.iter().sum();
    let anna_share = (total_cost - bill[k as usize]) / 2;

    // If Anna paid the correct amount, print "Bon Appetit", otherwise print the amount Brian owes her
    if b == anna_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b - anna_share);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // First line: n (number of items), k (index of item Anna didn't eat)
    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let _n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    // Second line: the bill items
    let bill: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Third line: amount Anna paid
    let b = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Call the bonAppetit function
    bonAppetit(&bill, k, b);
}