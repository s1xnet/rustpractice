#[test]
fn test1() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!'); // Adding '!' as a char to complete the sentence

    move_ownership(s);

    // This assertion will no longer be valid here because ownership has been moved.
    // assert_eq!(s, "hello, world!");

    println!("Success!");
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}


#[test]
fn test2() {
    let mut s = String::from("hello, world");

    // Full slice of the string
    let slice1: &str = &s; // or &s[..]
    assert_eq!(slice1, "hello, world");

    // Partial slice up to index 5
    let slice2 = &s[0..5];
    assert_eq!(slice2, "hello");

    // Defining `slice3` as a mutable String reference
    let slice3: &mut String = &mut s;
    slice3.push('!'); // Modifying the String
    assert_eq!(slice3, "hello, world!");

    println!("Success!");
}

#[test]
fn test3() {

    let s: String = String::from("hello, world!");

    let slice: &str = &s;

    let s: String = slice.to_string();

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

#[test]
fn test4() {
    let s = String::from("hello, 世界");

    // Access the first character 'h' using slicing
    let slice1 = &s[0..1]; // We slice one byte (UTF-8), so it's just "h"
    assert_eq!(slice1, "h");

    // Access the character "世" using the char_indices iterator
    // Use char_indices to find the correct byte range for "世"
    let slice2 = &s[7..10]; // Correctly slice "世" (starts at byte 7, ends at byte 10)
    assert_eq!(slice2, "世");

    // Iterate through all chars in the string
    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世');
        }
    }

    println!("Success!");
}


#[test]
fn test5() {
    let mut s = String::new();

    // Add some initial content to the string
    s.push_str("hello");

    // Some bytes, in a vector
    let v = vec![104, 101, 108, 108, 111];

    // Turn a byte's vector into a String
    let s1 = String::from_utf8(v).unwrap(); // This will convert the bytes to a string

    assert_eq!(s, s1); // Assert that the two strings are equal

    println!("Success!");
}


#[test]
fn test6() {
    let mut s = String::new();

    // Reserve capacity for 10 bytes (2 * "hello" = 10 bytes)
    s.reserve(10);

    // Check initial capacity
    println!("{}", s.capacity());

    // Add strings in the loop
    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());  // Print the capacity after each push
    }

    println!("Success!");
}


#[test]
use std::mem;

fn test7() {
    let story = String::from("Rust By Practice");

    // Prevent automatically dropping of the String's data
    let mut story = mem::ManuallyDrop::new(story);

    // Access raw pointer, length, and capacity
    let ptr = story.as_mut_ptr();      // Get the raw pointer to the string's data
    let len = story.len();             // Get the length of the string (number of bytes)
    let capacity = story.capacity();   // Get the total capacity of the string

    assert_eq!(16, len);

    // We can rebuild a String out of ptr, len, and capacity. This is all
    // unsafe because we are responsible for making sure the components are valid:
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(*story, s);

    println!("Success!");
}

//////////////

#[test]
fn test8() {
    let arr: [u8; 3] = [1, 2, 3];

    let v = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3];
    is_vec(&v);

    // vec!(..) and vec![..] are same macros, so
    let v = vec!(1, 2, 3);
    is_vec(&v);

    // Use Vec::new and `for` to rewrite the below code
    let mut v1: Vec<[u8; 3]> = Vec::new();
    for _ in 0..1 {
        v1.push(arr);  // Push the array into the vector
    }
    is_vec(&v1);

    assert_eq!(v, v1);

    println!("Success!");
}

fn is_vec(v: &Vec<u8>) {}


#[test]
fn test9() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);

    let mut v2 = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    assert_eq!(v1, v2);

    println!("Success!");
}


#[test]
fn test10() {
    // Array -> Vec
    // impl From<[T; N]> for Vec
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr); // Fill in: `Vec::from(arr)`
    let v2: Vec<i32> = arr.to_vec(); // Fill in: `arr.to_vec()`

    assert_eq!(v1, v2);

    // String -> Vec
    // impl From<String> for Vec
    let s = "hello".to_string();
    let v1: Vec<u8> = s.into_bytes(); // Fill in: `s.into_bytes()`

    let s = "hello".to_string();
    let v2 = s.into_bytes();
    assert_eq!(v1, v2);

    // impl<'_> From<&'_ str> for Vec
    let s = "hello";
    let v3 = s.as_bytes().to_vec(); // Fill in: `s.as_bytes().to_vec()`
    assert_eq!(v2, v3);

    // Iterators can be collected into vectors
    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]);

    println!("Success!");
}


#[test]
fn test11() {
    let mut v = Vec::from([1, 2, 3]);

    // Print the current elements of v
    for i in 0..v.len() {
        println!("{:?}", v[i]);
    }

    // Add values to the vector
    for i in 0..3 {
        v.push(v[i] + 1); // Push the incremented value of v[i]
    }

    // Check the result
    assert_eq!(v, vec![1, 2, 3, 2, 3, 4]);

    println!("Success!");
}


#[test]
fn test12() {
    let mut v = vec![1, 2, 3];

    // Get a slice of the entire vector
    let slice1 = &v[..]; // This is fine: it references all elements of the vector
    let slice2 = &v[0..v.len()]; // Use `v.len()` to prevent out-of-bounds error

    assert_eq!(slice1, slice2);

    // Slices are read-only
    // We can modify the vector itself, but not the slice
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4); // This will add 4 to the vector

    // Now we take a mutable slice, which still does not have `push` capability
    let slice3 = &mut v[0..3]; // Get a mutable slice of the first 3 elements
    slice3[2] = 4; // Modify the slice, not using `push`

    assert_eq!(slice3, &[1, 2, 4]); // Check if the slice is updated correctly

    println!("Success!");
}


#[test]
fn test13() {
    let mut vec = Vec::with_capacity(10);

    // The vector contains no items, even though it has capacity for more
    assert_eq!(vec.len(), 0); // Initially, the length is 0
    assert_eq!(vec.capacity(), 10); // The capacity is 10

    // These are all done without reallocating...
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10); // After pushing 10 elements, the length is 10
    assert_eq!(vec.capacity(), 10); // The capacity remains 10, no reallocation

    // ...but this may make the vector reallocate
    vec.push(11);
    assert_eq!(vec.len(), 11); // After pushing 11, the length is 11
    assert!(vec.capacity() >= 11); // The capacity should be >= 11 due to reallocation

    // Fill in an appropriate value to make the `for` done without reallocating
    let mut vec = Vec::with_capacity(100); // Pre-allocate enough space for 100 elements
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100); // The length after pushing 100 elements is 100
    assert_eq!(vec.capacity(), 100); // The capacity should be 100, no reallocation

    println!("Success!");
}


#[test]
#[derive(Debug, PartialEq)] // Deriving PartialEq to allow comparisons
enum IpAddr {
    V4(String),
    V6(String),
}

fn test14() {
    // FILL in the blank
    let v: Vec<IpAddr> = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    // Comparing two enums need to derive the PartialEq trait
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!");
}


#[test]
trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0);
    }
}

struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0);
    }
}

fn test15() {
    // FILL in the blank
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}

/////////////////////

#[test]
use std::collections::HashMap;

fn test16() {
    let mut scores = HashMap::new();

    // Insert all values as f64
    scores.insert("Sunface", 98.0);        // Inserting integer as f64
    scores.insert("Daniel", 95.0);         // Inserting integer as f64
    scores.insert("Ashley", 69.0);         // Inserting float
    scores.insert("Katie", 58.0);          // Convert string to float

    // Get returns an Option<&V>
    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98.0));  // We need to match the value type (i.e., reference to 98.0)

    if scores.contains_key("Daniel") {
        // Indexing returns a value V, we can use the value directly here
        let score = scores["Daniel"];
        assert_eq!(score, 95.0);  // "Daniel" has the value 95.0, a float
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);  // After removing "Daniel", there should be 3 entries left

    for (name, score) in &scores {
        println!("The score of {} is {}", name, score);
    }
}


#[test]
use std::collections::HashMap;

fn test17() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // Approach 1: Using the `collect` method
    let teams_map2: HashMap<_, _> = teams.iter().cloned().collect();

    // Approach 2: Using a for loop
    let mut teams_map3 = HashMap::new();
    for team in &teams {
        teams_map3.insert(team.0, team.1);
    }

    // Asserting both maps are equal
    assert_eq!(teams_map1, teams_map2);
    assert_eq!(teams_map1, teams_map3);

    println!("Success!");
}


#[test]
use std::collections::HashMap;

fn test18() {
    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<&str, u8>` in this example).
    let mut player_stats = HashMap::new();

    // Insert a key only if it doesn't already exist
    player_stats.entry("health").or_insert(100);

    assert_eq!(player_stats["health"], 100);  // Value should be 100

    // Insert a key using a function that provides a new value only if it
    // doesn't already exist
    player_stats.entry("health").or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"], 100);  // Value should still be 100, since it wasn't replaced

    // Ensures a value is in the entry by inserting the default if empty, and returns
    // a mutable reference to the value in the entry.
    let health = player_stats.entry("health").or_insert(50);
    assert_eq!(*health, 100);  // Value should still be 100, no change yet
    *health -= 50;
    assert_eq!(*health, 50);   // After subtracting 50, value should be 50

    println!("Success!");
}

fn random_stat_buff() -> u8 {
    // Could actually return some random value here - let's just return
    // some fixed value for now
    42
}


#[test]
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq, Hash)]  // Automatically implements Eq, PartialEq, and Hash
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    /// Creates a new Viking.
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn test19() {
    // Use a HashMap to store the vikings' health points.
    let mut vikings = HashMap::new();  // Using HashMap::new for mutable map
    vikings.insert(Viking::new("Einar", "Norway"), 25);
    vikings.insert(Viking::new("Olaf", "Denmark"), 24);
    vikings.insert(Viking::new("Harald", "Iceland"), 12);

    // Use derived implementation to print the status of the vikings.
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}


#[test]
use std::collections::HashMap;

fn test20() {
    let v1 = 10;
    let mut m1 = HashMap::new();
    m1.insert(v1, v1);
    println!("v1 is still usable after inserting to hashmap : {}", v1);

    let v2 = "hello".to_string();
    let mut m2 = HashMap::new();
    // Ownership moved here, so we clone `v2` to keep it usable
    m2.insert(v2.clone(), v1);  // We clone v2 here instead of moving it

    // Now we can still use v2
    assert_eq!(v2, "hello");

    println!("Success!");
}