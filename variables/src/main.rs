const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

#[derive(Debug)]
struct Foo(&'static str);
impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    // single‐scope shadowing
    let a = Foo("first");
    let a = Foo("second");
    println!("now a = {:?}", a);
    // neither Foo("first") nor Foo("second") are dropped here—
    // they’ll both be dropped at the end of `main` in reverse order:
    // first "second", then "first"

    // you cannot do `drop(a);` and then magically get back the "first" Foo,
    // because the name `a` was rebound to the "second" Foo and the "first" is hidden.

    // nested‐scope shadowing
    let b = Foo("outer");
    {
        let b = Foo("inner");
        println!("inside block, b = {:?}", b);
    } // here Foo("inner") is dropped
    println!("after block, b = {:?}", b); // refers to Foo("outer")
}