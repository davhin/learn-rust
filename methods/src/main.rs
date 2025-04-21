#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 10
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    if rect1.width() {
        println!(
            "The rectangle width is larger than 10 and the area of the rectangle is {} square pixels",
            rect1.area()
        );
    } else {
        println!("The rectangle does not have width > 10")
    }
    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };
    let rect3 = Rectangle {
        width: 40,
        height: 30,
    };
    if rect1.can_hold(&rect2) {
        println!("Rect 1 can hold rect 2")
    } else {
        println!("Rect 1 cannot hold rect 2")
    }
    if rect1.can_hold(&rect3) {
        println!("Rect 1 can hold rect 3")
    } else {
        println!("Rect 1 cannot hold rect 3")
    }
    let square1 = Rectangle::square(50);
}
