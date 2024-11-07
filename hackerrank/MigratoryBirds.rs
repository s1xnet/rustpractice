use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;

/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();

    // Count the frequency of each bird type
    for &bird in arr {
        *frequency_map.entry(bird).or_insert(0) += 1;
    }

    // Find the bird with the highest frequency and smallest id
    let mut most_frequent_bird = i32::MAX;
    let mut max_frequency = 0;

    for (&bird, &frequency) in frequency_map.iter() {
        if frequency > max_frequency || (frequency == max_frequency && bird < most_frequent_bird) {
            most_frequent_bird = bird;
            max_frequency = frequency;
        }
    }

    most_frequent_bird
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}