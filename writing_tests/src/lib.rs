#![allow(unused)]


pub fn add(left: usize, right: usize) -> usize {
    left + right + 0
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
    // String::from("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

fn function(x: usize) -> Result<usize, String> {
    match x {
        0 => Err(String::from("error in function; got 0")),
        other => Ok(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        // panic!("Make this test fail");
    }

    #[test]
    fn larger_rectangle_can_hold_smaller() {
        let larger = Rectangle { width: 10, height: 20 };
        let smaller = Rectangle { width: 10, height: 20 };
        assert!(larger.can_hold(&larger));
        assert!(larger.can_hold(&smaller));
        assert!(smaller.can_hold(&larger));
    }

    #[test]
    fn smaller_rectangle_cannot_hold_larger() {
        let smaller = Rectangle { width: 10, height: 20 };
        let larger = Rectangle { width: 20, height: 10 };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn should_add_two() {
        assert_eq!(add_two(3), 5);
        assert_ne!(add_two(3), 6);
    }

    #[test]
    fn greet_name() {
        let result = greet("Ana");
        assert!(
            result.contains("Ana"),
            "Greeting does not contain name; value received is: \"{}\"",
            result
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn guess_greater_than_100() {
        Guess::new(100 + 1);
    }

    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1, got 0.")]
    fn guess_less_than_1() {
        Guess::new(1 - 1);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4, "2 + 2 != 4; result = {}", result);
    }

    #[test]
    fn it_works_result() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(format!("two plus two does not equal four; result = {}", result))
        }
    }

    #[test]
    fn function_returns_error_1() {
        assert!(function(0).is_err());
    }

    #[test]
    fn function_returns_error_2() {
        assert!(match function(0) {
            Ok(_) => false,
            Err(_) => true,
        });
    }

    #[test]
    #[should_panic]
    fn function_returns_error_3() {
        function(0).unwrap();
    }

    #[test]
    fn function_returns_ok_1a() {
        assert!(!function(77).is_err());
    }

    #[test]
    fn function_returns_ok_1b() {
        assert!(function(77).is_ok());
    }

    #[test]
    fn function_returns_ok_2() {
        assert!(match function(77) {
            Ok(_) => true,
            Err(_) => false,
        });
    }

    #[test]
    fn function_returns_ok_3() {
        function(77).unwrap();
    }
}
