#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn new(height: u32, width: u32) -> Self {
        Self { height, width }
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height < other.height
    }

    fn print(&self) {
        println!("{:#?}", self);
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(45, 60);

    println!("The area of the rectangle is {}.", rect1.area());
    rect1.print();

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
