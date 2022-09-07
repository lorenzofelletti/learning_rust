use rectangles::Rectangle;

fn main() {
    let scale = 2;
    let rect = Rectangle::build_rectangle(dbg!(30 * scale), 50);

    dbg!(&rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!(
        "Is the rectangle's width 0? {}",
        dbg!(rect.has_zero_width())
    );

    let other = Rectangle::square(dbg!(10 * scale));

    dbg!(&other);

    println!(
        "Can the rectangle hold the other rectangle? {}",
        dbg!(rect.can_hold(&other))
    );
}
