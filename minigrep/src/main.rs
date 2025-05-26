use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path: &String = &args[2];
    println!("Looking for {query} in {file_path}");
    let contents = fs::read_to_string(file_path).expect(
        "should \
    have been able to read the file",
    );
    println!("Full file: \n {contents}");
}
