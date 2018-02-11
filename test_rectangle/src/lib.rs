#[derive(Debug, PartialEq)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn from(width: u32, length: u32) -> Rectangle {
        Rectangle {
            width,
            length,
        }
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100, got 200.")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn test_greeting() {
        let result = greeting("Sarah");

        assert!(
            result.contains("Sarah"),
            "Result should contain `Sarah` got `{}` instead",
            result
        );
    }

    #[test]
    fn larger_can_hold_smaller() {
        let large = Rectangle::from(8, 10);
        let small = Rectangle::from(4, 5);

        assert!(large.can_hold(&small));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let large = Rectangle::from(8, 10);
        let small = Rectangle::from(4, 5);

        assert!(!small.can_hold(&large));
    }

    #[test]
    fn from() {
        assert_eq!(
            Rectangle::from(10, 20),
            Rectangle {
                width: 10,
                length: 20,
            }
        )
    }
}
