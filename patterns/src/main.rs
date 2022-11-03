fn main() {
    // println!("Hello, world!");
    simple_pattern();
    simple_pop();
    simple_enumerate();
    simple_match();
    match_struct_fields();
}

fn simple_enumerate() {
    let v = vec!['a','b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn simple_pattern() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the bg");
    } else if is_tuesday {
        println!("Tuesday is here! using green!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the bg");
        } else {
            println!("Using orange as the bg");
        }
    }
    else {
        println!("Using blue as the bg");
    }
}

fn simple_pop() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn simple_match() {
    let x = 8;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 | 5 => println!("four or five"),
        6..=9 => println!("another integer"),
        _ => println!("anything"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn match_struct_fields() {
    let p = Point { x: 5, y: 10 };

    let Point { x, y } = p;
    assert_eq!(5, x);
    assert_eq!(10, y);

    match p {
        Point { x, y:0 } => println!("Origin on the y axis at {}", x),
        Point { x: 0, y } => println!("Origin on x axis at {}", y),
        Point { x, y } => println!("Origin not at either axis: ({}, {})", x, y),
    }
}
