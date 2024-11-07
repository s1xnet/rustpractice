#[test]
fn test1() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}

#[test]
fn test2() {
    let n = 5;

    let big_n =
        if n < 10 && n > -10 {
            println!("{}, and is a small number, increase ten-fold", n);
            10 * n // This returns an integer value
        } else {
            println!("{}, and is a big number, halve the number", n);
            n / 2 // This division now returns an integer result
        };

    println!("{} -> {}", n, big_n);
}

#[test]
fn test3() {
    for n in 1..100 { // Modify this line to exclude 100
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
}

#[test]
fn test4() {
    let names = [String::from("liming"), String::from("hanmeimei")];
    for name in &names {
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    for n in numbers {
        // Do something with n...
    }

    println!("{:?}", numbers); // This works because i32 implements the Copy trait
}

#[test]
fn test5() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i + 1, v);
    }
}

#[test]
fn test6() {
    // A counter variable
    let mut n = 1;

    // Loop while the condition is true
    while n <= 10 {  // Condition for the loop to continue as long as n is 10 or less
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;  // Increment the counter to avoid an infinite loop
    }

    println!("n reached {}, so loop is over", n);
}

#[test]
fn test7() {
    let mut n = 0;
    for i in 0..=100 {
        if n == 66 {
            break;  // Break the loop when n equals 66
        }
        n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");
}

#[test]
fn test8() {
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n += 1;
            continue;  // Skip the rest of the loop and move to the next iteration
        }

        // This code runs only when n == 66
        println!("n is {}", n);
    }

    assert_eq!(n, 66);

    println!("Success!");
}

#[test]
fn test9() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;  // Continue to the next iteration of the loop
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;  // Exit the loop
        }
    }

    assert_eq!(count, 5);

    println!("Success!");
}

#[test]
fn test10() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break 20;  // Break out of the loop and return 20
        }
    };

    assert_eq!(result, 20);

    println!("Success!");
}

#[test]
fn test11() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // This would break only the inner1 loop
                break 'inner1; // `break` is also works.
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                // This breaks the outer loop
                break 'outer;
            }

            // This will continue the outer loop
            continue 'outer;
        }
    }

    assert!(count == 30);  // The final value of count will be 30

    println!("Success!");
}