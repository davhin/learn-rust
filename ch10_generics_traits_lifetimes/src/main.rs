use ch10_generics_traits_lifetimes::{NewsArticle, SocialPost, Summary};

fn largest_i32(numbers: &[i32]) -> &i32 {
    let mut largest = &numbers[0];
    for number in &numbers[1..] {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_char(chars: &[char]) -> &char {
    let mut largest = &chars[0];
    for number in &chars[1..] {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for value in &list[1..] {
        if value > largest {
            largest = value;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec!['a', 'c', 'd', 'Z'];
    let result = largest(&number_list);
    println!("The largest char is {}", result);

    // now use the traits implemented in the lib
    let post = SocialPost {
        username: String::from("bjhorseman"),
        content: String::from("good lord that's depressing"),
        reply: true,
        repost: false,
    };
    println!("1 new social post: {}", post.summarize());
}
