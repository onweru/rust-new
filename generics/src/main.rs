use generics::{Summary, Tweet, NewsArticle, notify};

fn main() {
    // println!("Hello, world!");
    // without generics
    let numbers = vec![53, 18, 9, 678, 97, 83];
    let characters = vec!['a', 'l', 'k', 'x', 'w', 'z'];

    let boundary_numbers = min_max(&numbers);
    println!("The smallest number is {}", boundary_numbers.0);
    println!("The largest number is {}", boundary_numbers.1);

    // with generics
    let boundary_characters = generic_min_max(&characters);
    println!("The smallest character is {}", boundary_characters.0);
    println!("The largest character is {}", boundary_characters.1);

    let origin = Point {x: 0, y:0};
    let random_point = Point {x: 7.1, y:9.18};

    println!("({},{}) is the origin on a graph", origin.x(), origin.y);

    let distance_to_random_point = random_point.distance_from_origin();

    println!("The distance from origin to the stated random point is: {}", distance_to_random_point);

    // generic struct with multi types
    let coordinate1 = Coordinate {x: 29.9792, y: "North" };
    let coordinate2 = Coordinate {x: 31.1342, y: "East" };

    let coordinate_mixup = coordinate1.mixup(coordinate2);

    println!("False: the pyramids of Giza cannot be found on {}° East", coordinate_mixup.x);

    // traits
    let user = String::from("Weru");

    let tweet = Tweet {
        username: &user,
        content: String::from ("A mere tweet, bravo!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Why rust is fast"),
        content: String::from ("Rust is a compiled language whose speed is almost comparable to C's"),
        location: String::from("Nairobi"),
        author: &user,
    };

    let tweet_summary = print_summary(&tweet, String::from("tweet"));
    let oped_summary = print_summary(&article, String::from("opinion article"));

    println!("{}", tweet.content());
    println!("{}", oped_summary);
    println!("{}", tweet_summary);

    // traits as parameters
    notify(&article);
    // this👇 function call should fail as 32 is i32 and doesn't implement the Summary trait
    // notify(32);

    // lifetimes' annotations
    let string1 = String::from("wxyz");
    let string2 = "klm";

    longest_string(&string1.as_str(), &string2);


}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Coordinate<X, Y> {
    x: X,
    y: Y,
}

impl <X, Y> Coordinate<X, Y> {
    fn mixup<X1, Y1>(self, other: Coordinate<X1, Y1>) -> Coordinate<X, Y1> {
        Coordinate {
            x: self.x,
            y: other.y,
        }
    }
}

fn min_max(list: &[i32]) -> (&i32, &i32) {
    let mut max = &list[0];
    let mut min = &list[0];

    for number in list {
        if number > max {
            max = number;
        }
        if number < min {
            min = number;
        }
    }

    (min, max)
}

fn generic_min_max<T: std::cmp::PartialOrd> (list: &[T]) -> (&T, &T) {
    let mut max = &list[0];
    let mut min = &list[0];

    for item in list {
        if item > max {
            max = item;
        }
        if item < min {
            min = item;
        }
    }

    (min, max)
}

fn print_summary<T: Summary>(media: &T, name: String) -> String {
    format!("1 new {}: {}", name, media.summarize())
}

fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}