use example::{ * };
fn main() {
    display_struct();
    display_cities();

}

fn display_struct() {
    // dirty function used for practice
    println!("{:?}", UnPrintable{number: 2});

    println!("{0:?} {1:?}", "help", "thanks");

    let name = "Peter";
    let age = 19;

    let peter = Person {name, age};
    println!("{:#?}, {}, {}", peter, peter.name, peter.age);

    let minmax = MinMax(0, 14);
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {} and the smaill range is {}", big_range, small_range);

    let point = Point2D { x: 3.3, y : 7.2 };

    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex_num = Complex { real: 3.3, imag: 9.3};

    println!("Display: {}", complex_num);
    println!("Debug: {:?}", complex_num);

    let vec_list = List(vec![3, 5, 9]);

    println!("Display: {}", vec_list);
}

fn display_cities() {
    let oslo = City { name: "Oslo", lat: 59.95, lon: 10.75 };
    let dublin = City { name: "Dublin", lat: 53.347778, lon: -6.259722 };

    let cities = [oslo, dublin];

    for city in cities.iter() {
        println!("{}", city);
    }
}