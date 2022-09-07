#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn build_rectangle(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let scale = 2;
    let rect = Rectangle::build_rectangle(dbg!(30 * scale), 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );

    dbg!(&rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
