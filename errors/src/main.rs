use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // println!("Hello, world!");
    let hey = "hey.txt".to_string();
    let hola = "hola.txt".to_string();

    handle_errors_with_match(&hola);
    handle_errors_with_closures(&hey);
    handle_errors_with_expect(&hola);
}

// enum Action {
//     Create(String),
//     Open(String),
//     Read(String),
//     Write(String),
// }

fn handle_errors_with_match(filename: &String) {
    let hello_file_result = File::open(&filename);
    let _hello_file = match hello_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&filename) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem create the {} file: {:?}", &filename, e),
            },
            other_error => {
                panic!("Problem opening the {}: {:?}", &filename, other_error);
            }
        }
    };
}

fn handle_errors_with_closures(filename: &String) {
    let _hello_file = File::open(filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filename).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn handle_errors_with_expect(filename: &String) {
    let mut error = "doesn't exist yet".to_string();
    error = format!("{} {}", &filename, error);
    let _file = File::open(&filename).expect(&error);
    // println!("{}", &error);
}