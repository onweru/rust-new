fn main() {
    // println!("Hello, world!");
    let _v: Vec<i32> = Vec::new();

    let mut new_v = vec![1, 2, 3];

    new_v.push(20);
    new_v.push(11);
    new_v.push(36);
    new_v.push(26);
    new_v.push(61);

    let fifth: &i32 = &mut new_v[4];
    println!("The fifth element is {}", &fifth);

    let seventh: Option<&i32> = new_v.get(6);

    match seventh {
        Some(seventh) => println!("The seventh element is {}", seventh),
        None => println!("The is no seventh element."),
    }

    for num in new_v {
        println!("{}", num);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // hash map (k, v)
    
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Red");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Score is: {}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Maroon");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field-value are invalid here

}