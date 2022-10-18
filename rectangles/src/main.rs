fn main() {
    // println!("Hello, world!");
    // calculate area without a struct
    let width = 100;
    let height = 25;

    println!("The area of this rectangle is: {}", calculate_area(width, height));

    // use a rectangle tuple to obtain the area

    let rect = (80, 35);
    println!("The area of this rectangle is: {}", calculate_area_with_tuple(rect));

    // use a rectangle struct to obtain the area

    let new_rect = Rectangle {
        width: 130,
        height: 90
    };
    let old_rect = Rectangle {
        width: 120,
        height: 70
    };

    println!("The area of this rectangle is: {}", calculate_area_with_struct(&new_rect));

    println!("{}", new_rect.width);


    println!("{:#?}", new_rect);

    dbg!(&new_rect);

    // calculate area using a struct method

    println!("The area of this rectangle is {}", new_rect.area());

    // compare rectangles fit

    println!("Can the first rectangle fit into the second? {}", new_rect.can_hold(&old_rect));

    // try associated function
    let first_square = Rectangle::square(360);
    dbg!(&first_square);

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        let has_larger_width = self.width >= other.width;
        let has_larger_height = self.height >= other.height;

        has_larger_height && has_larger_width
    }

    // associated function

    fn square(width: u32) -> Self {
        Self {
            width,
            height: width,
        }
    }
}

fn calculate_area(width: u32, height: u32) -> u32 {
    height * width
}

fn calculate_area_with_tuple(dimensions: (u32, u32)) -> u32 {
   dimensions.0 *dimensions.1
}

fn calculate_area_with_struct (rect: &Rectangle) -> u32 {
    rect.width * rect.height
}