pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail!");
    // }

    #[test]
    fn larger_rect_holds_smaller() {
        let big_r = Rectangle {
            width: 10,
            height: 6,
        };
        let small_r = Rectangle {
            width: 8,
            height: 4,
        };
        assert!(big_r.can_hold(&small_r));
        assert!(!small_r.can_hold(&big_r));
    }
}
