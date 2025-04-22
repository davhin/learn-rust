enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main2() {
    println!("Hello, world!");
    println!("Today, we are learning about collections, starting with vectors");
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3]; // this conveniently uses the vec! macro to define the vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    // v.push(7.5); // this fails since Rust has inferred the type to be i32 from the previous lines
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // also works without ref, but probably creates ownership issues?
    println!("The third element of v is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("v has a third value and it is {third}"),
        None => println!("No third value in v"),
    }
    // let does_not_exist = &v[100]; // this will panic at runtime
    let does_not_exist = v.get(100); // this will not panic, but return None

    let mut v = vec![100, 200, 500];
    for i in &v {
        println!("{i}")
    }
    for i in &mut v {
        *i = *i + 50;
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(4.5),
        SpreadSheetCell::Text(String::from("Hallo!")),
    ];
}

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let team_score = scores.get(&team_name).copied().unwrap_or(0);
    for (key, value) in &scores {
        println!("{key}: {value}")
    }
}
