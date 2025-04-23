use std::collections::HashMap;

fn main() {
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
}

fn get_median(v: &Vec<i32>) -> f32 {
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
