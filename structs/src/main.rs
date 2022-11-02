use structs::AveragedCollection;

fn main() {
    // println!("Hello, world!");

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut first_user = User {
        username: String::from("Weru"),
        email: String::from("heyweru@icloud.com"),
        active: false,
        sign_in_count: 3,
    };

    first_user.email = String::from("hi@weru@icloud.com");

    println!("Hi my name is {0}, my email is {1}, and I have logged in {2} times", first_user.username, first_user.email, first_user.sign_in_count);

    let new_video = edit_video(String::from("Fibonacci"), 30);

    println!("{0} is making his {1}th  {2}video, it's called {3}", first_user.username, new_video.id, new_video.kind, new_video.name);

    // let second_user = User {
    //     active: first_user.active,
    //     username: String::from("dan"),
    //     email: first_user.email,
    //     sign_in_count: 4,
    // };

    let second_user = User {
        username: String::from("dan"),
        email: String::from("heyweru@icloud.com"),
        ..first_user
    };

    println!("{0} second name is {1}, {2}", first_user.username, second_user.username, second_user.active);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // let mut wealth = AveragedCollection::new();

    // let avg_wealth = try_collection_struct(wealth);
    // println!("average wealth is {}", avg_wealth);

}

struct Video {
    name: String,
    id: u32,
    kind: String,
}

fn edit_video(name: String, id: u32) -> Video {
    Video {
        name,
        id,
        kind: String::from("mp4"),
    }
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn try_collection_struct(obj: AveragedCollection) -> f64 {
    obj.average()
}