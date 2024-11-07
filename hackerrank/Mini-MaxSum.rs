use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */
fn miniMaxSum(arr: &[i32]) {
    let total_sum: i32 = arr.iter().sum(); // Sum of all elements in the array

    let mut min_sum = i32::MAX;
    let mut max_sum = i32::MIN;

    // For each element, calculate the sum of the remaining elements
    for &num in arr {
        let current_sum = total_sum - num;
        min_sum = min_sum.min(current_sum); // Update the min_sum
        max_sum = max_sum.max(current_sum); // Update the max_sum
    }

    // Print the result in the required format
    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Read the array of integers
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Call the miniMaxSum function to calculate and print the result
    miniMaxSum(&arr);
}
