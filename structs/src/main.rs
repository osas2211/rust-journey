#[allow(dead_code)]
struct User {
    name: String,
    sign_in_count: u32,
    email: String,
    username: String,
}

// Tuple Struct
#[allow(unused)]
struct Color(u8, u8, u8);

#[allow(unused)]
fn main() {
    // let mut user1: User = User {
    //     name: String::from("Osaretin Frank"),
    //     email: String::from("osaretinmail@mail.com"),
    //     sign_in_count: 0,
    //     username: String::from("classic_dev"),
    // };

    // user1.email = String::from("changed@mai.com");
    // println!("{}", user1.email);

    // let black: Color = Color(0, 0, 0);
    rect_lesson();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn rect_lesson() {
    let scale: u32 = 2;
    let rect1 = Rectangle {
        height: 20,
        width: dbg!(scale * 40),
    };
    let rect_area = area(&rect1);
    println!("Rectangle Area: {}", rect_area);
    println!("{:#?}", rect1);
    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
