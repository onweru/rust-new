fn main() {
    let number = double_add_three_result(add_three, 4);
    println!("new number is: {}", number);
}

fn add_three(x: i32) -> i32 {
    x + 3
}

fn double_add_three_result(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// fn factorial(f: fn(u32) -> u32,  arg: u32) {
    // n! = n × (n−1)!
    // let mut product = 0;
    // let upper_bound = arg + 1
    // for num in 0..upperbound { {
    //     product = num * f(num - 1)
    // }
// }
