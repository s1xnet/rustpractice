#[test]
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let num1 = 56;
    let num2 = 98;

    let result = gcd(num1, num2);
    println!("The greatest common divisor of {} and {} is {}", num1, num2, result);
}