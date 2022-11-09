use std::{ fmt, slice };
use std::ops::Add;

static mut COUNTER: u32 = 0;

fn main() {
    unsafe_split();

    let mut v = vec![1, 2, 3, 4];

    let mid: usize = 2;

    let _split_v_result = custom_split(&mut v, mid);

    // println!("{:#?}", split_v_result);
    add_to_count(7);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    assert_eq!(
        Point { x: 2, y: 3 } + Point { x: 5, y: 0 },
        Point { x: 7, y: 3 }
    );

    make_him_fly();

    print_dog_baby_name();
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
fn custom_split(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid), 
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }

}

fn unsafe_split() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    // println!("{:#?}", &mut v);

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    // println!("{:#?}", a);
    // println!("{:#?}", b);
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("umh, waving arms furiously");
    }
}

fn make_him_fly() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn print_dog_baby_name() {
    println!("A baby dog is called a {}\n", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}