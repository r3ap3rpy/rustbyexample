use std::todo;

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32,u32,u32),
    HSV(u32,u32,u32),
    HSL(u32,u32,u32),
    CMY(u32,u32,u32),
    CMYK(u32,u32,u32),
}

fn main() {
    //let color = Color::RGB(10,20,30);
    let color = Color::HSV(10,20,30);
    match color {
        Color::RGB(a,b,c) => println!("RGB({}, {}, {})",a,b,c),
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        _ => todo!("Rest of the colors need to be implemented!"),
    }
}
