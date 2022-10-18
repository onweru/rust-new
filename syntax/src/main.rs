use std::io;
fn main() {
    // println!("Hello, world!");

    /*
        multi-line comment

        regular variable which can be made mutable
    */

    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // constants

    /*
        - uppercase and screaming snake case
        - must annotate type
    */

    const SECONDS_PER_DAY: u32 = 60 * 60 * 24;

    println!("Every day has {SECONDS_PER_DAY} seconds");

    // variable shadowing

    /*
        For shadowing to happen, the variable must be redeclared i.e using the `let` keyword.

        By shadowing, the variable name can be used with a different type value
    */

    let x = x * 2;

    {
        let x: i32 = x - 14;
        println!("The even newer x value is: {x}");

        let x = "x factor";

        println!("The final form of x is: {x}");
    }

    println!("The new x value is: {x}");

    // data types

    /*
    Rust has scalar and compound values.

    Of the sclaar types: there are integers, floats, boolean and characters.

    compound types include 1) tuples and 2) arrays


    */

    let guess: u32 = "53".parse().expect("Not a number!");
    println!("You guessed: {guess}");

    let difference = 85.3 - 86.7;
    println!("The difference is: {difference}");

    let quotient: f32 = 95.0 / 7.0;
    println!("The quotient is: {quotient}");

    let quotient: i32 = 95 / 7;
    println!("The quotient is: {quotient}");

    let quotient_is_a_float: bool = false;
    println!("The last quotient is a float? {quotient_is_a_float}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("Please call {c}{z}, please {heart_eyed_cat}");

    let tup = (32, 54.6, 'C');

    let (n,y,s) = tup; // destructuring a tuple

    println!("A string from tuple looks like: {s}");
    println!("Other values from a tuple looks like: {y}, {n}");

    let new_float = tup.1;

    println!("Float from tuple is: {new_float}");

    let months = ["January","February","March","April","May","June","July","August","September","October","November","December"];

    println!("Please enter a month number");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = months[index];

    println!("The month at index {index} is: {element}");

    // functions
   simple_function();
   add_numbers(32, -23);

   let modulo = custom_modulo(-97, 10);
   println!("The modulo of 97 and 10 is: {modulo}");

   // control flow

   let condition = true;
   let number = if condition { 5 } else { 6 };

   println!("The value of iffy number is {number}");

   exit_loop(23);

   labelled_loops();

   conditional_while_loop(34);

       precise_loop(&months);

    farenheit_to_celcius(56);
}

fn simple_function() {
    println!("Hey, this is a simple function");
}

fn add_numbers(x: i32, y: i32) -> i32 {
    /*
        parameters in rust functions must contain type annotations
    */
    let sum =  x + y;
    println!("The sum of {x} and {y} is: {sum}");
    sum
}

fn custom_modulo(x: i32, y: i32) -> i32 {
    let quotient = x / y;
    let modulo = x - (quotient * y);
    modulo
}

fn exit_loop(x: u32) -> u32 {
    // return integer after x number of loops
    let mut counter = 0;
    let mut sum = 0;

    let result = loop {
        counter += 1;
        sum += x;

        if counter == x {
            break sum;
        }
    };

    println!("The result of the simple loop is {result}");

    result
}

fn labelled_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn conditional_while_loop(x: u32) {
    let mut number = x;
    let mut counter = 0;

    while number >= 3 {
        counter += 1;
        number -= 3;
        println!("{number}!");
    }

    println!("Closing loop after {counter} rounds!");
}

fn precise_loop(list: &mut [str; 12]) {
    let mut counter = 1;
    for month in list {
        counter += 1;
        println!("Month {counter} is {month}");
    }
}

fn farenheit_to_celcius(temp: i32) -> f64 {
    let celcius = ((temp - 32) as f64) * 0.5555556;
    println!("{temp}°F equals {celcius}°C");
    celcius
}