fn main() {
    // println!("Hello, world!");
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user1_preference = Some(ShirtColor::Red);
    let user1_giveaway = store.giveaway(user1_preference);

    println!(
        "The user with preference {:?} gets {:?}",
        user1_preference, user1_giveaway
    );

    let user2_preference = None;
    let user2_giveaway = store.giveaway(user2_preference);

    println!(
        "The user with preference {:?} gets {:?}",
        user2_preference, user2_giveaway
    );

    let mut num_of_sorts = 0;

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| {
        num_of_sorts += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_of_sorts} operations", list);
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, preference: Option<ShirtColor>) -> ShirtColor {
        preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_shirts = 0;
        let mut blue_shirts = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => red_shirts += 1,
                ShirtColor::Blue => blue_shirts += 1,
            }

        }
        if red_shirts > blue_shirts {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
