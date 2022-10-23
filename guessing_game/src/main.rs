use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please use a real number!");
                continue;
            },
        };

        println!("You guessed: {guess}");
        Guess::new(guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again"),
            Ordering::Greater => println!("Too big! Try again"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be 1 ~ 100, got {},", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}