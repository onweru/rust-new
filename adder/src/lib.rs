pub fn add(left: usize, right: usize) -> usize {
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

pub fn add_three(a: i32) -> i32 {
    a + 3
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    // String::from("Hello")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be >= to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be <= 100, got {}", value);
        }

        Guess { value }
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

    #[test]
    #[ignore]
    fn another() {
        panic!("make the test fail");
    }

    #[test]
    fn larger_can_hold_tinier() {
        let larger = Rectangle {
            width: 80,
            height: 65,
        };

        let tinier = Rectangle {
            width: 50,
            height: 60,
        };

        assert!(larger.can_hold(&tinier));
    }

    #[test]
    fn tinier_cannot_hold_larger() {
        let larger = Rectangle {
            width: 40,
            height: 65,
        };

        let tinier = Rectangle {
            width: 15,
            height: 30,
        };

        assert!(!tinier.can_hold(&larger));
    }

    #[test]
    fn it_adds_three() {
        assert_eq!(add_three(2), 5);
    }

    #[test]
    // #[ignore]
    fn greeting_contains_name() {
        let carol = "Carol";
        let result = greeting(carol);
        assert!(
            result.contains(carol),
            "Greeting doesn't contain the value, {}",
            carol
        );
    }

    #[test]
    #[should_panic]
    fn too_big() {
        Guess::new(199);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 plus 2 != 4"))
        }
    }
}
