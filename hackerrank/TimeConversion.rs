use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */
fn timeConversion(s: &str) -> String {

    let hour: i32 = s[0..2].parse().unwrap();
    let minutes = &s[3..5];
    let seconds = &s[6..8];
    let period = &s[8..]; // Either "AM" or "PM"

    let converted_hour = match period {
        "AM" => {
            if hour == 12 {
                0 // Convert "12:xx:xxAM" to "00:xx:xx"
            } else {
                hour
            }
        }
        "PM" => {
            if hour == 12 {
                12 // Keep "12:xx:xxPM" as is
            } else {
                hour + 12 // Convert PM hours to 24-hour format
            }
        }
        _ => hour, // Fallback, though it should never reach here
    };
    
    format!("{:02}:{:02}:{:02}", converted_hour, minutes.parse::<i32>().unwrap(), seconds.parse::<i32>().unwrap())
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
