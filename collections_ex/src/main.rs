use std::collections::HashMap;
use std::io;

fn main() {
    // task 1: median and mode of a vector
    println!("Hello, world!");
    println!("Task 1: take a list of integers in a vector, and return the median and the mode");
    // let v: Vec<i32> = Vec::new();
    let v_even: Vec<i32> = vec![1, 1, 2, 3, 5, 8];
    let v_odd: Vec<i32> = vec![1, 1, 2, 3, 5, 8, 13];
    println!(
        "The mode of v_even is {} and the median is {}",
        get_mode(&v_even),
        get_median(&v_even)
    );
    println!(
        "The median of v_odd is {} and the mode is {}",
        get_median(&v_odd),
        get_mode(&v_odd)
    );
    // task 2: converting a string to pig latin
    println!("Task 2: Convert a string to pig latin");
    println!("Enter a string: ");
    // do calculation
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let pig_string: String = convert_to_pl(&s.trim());
    println!("The pig latin version is {pig_string}");
}

// task 3: with a hash map and vectors, create a text interface to allow a user to add emplooyee names to a department
// e.g. "Add David to engineering", then let user retrieve a list of all people in a department or all people in company
// by department, sorted alphabetically

// task 2: convert to pig latin
fn convert_to_pl(s: &str) -> String {
    let first: String = s.chars().nth(0).unwrap().to_string();
    let tail: String = s.chars().skip(1).collect();
    let result = tail + &first;
    result
}

// task 1: median and mode of a vector
fn get_median(v: &[i32]) -> f32 {
    let l: usize = v.len();
    let median: f32 = if l % 2 == 0 {
        let lower: &i32 = &v[l / 2 - 1];
        let upper: &i32 = &v[l / 2];
        ((*lower as f32) + (*upper as f32)) / 2.0
    } else {
        v[l / 2] as f32
    };
    median
}
fn get_mode(v: &Vec<i32>) -> i32 {
    let mut c: HashMap<i32, i32> = HashMap::new();
    for e in v {
        let count = c.entry(*e).or_insert(0);
        *count += 1;
    }
    let mut max_key: i32 = 0;
    let mut max_value: i32 = 1;
    for (key, value) in &c {
        if *value > max_value {
            max_value = *value;
            max_key = *key;
        }
    }
    max_key
}
