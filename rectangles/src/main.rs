
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.height >= rect.height && self.width >= rect.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }
}

fn main() {
    println!("Hello, world!");
    let width1 = 50;
    let height1 = 75;
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("There is a rectangle of {} square meters", area(width1, height1));
    let dimensions = (width1, height1);
    println!("There is a rectangle of {} square meters", area_tuple(dimensions));
    println!("Rectangle: {:#?}", rect1);
    println!("There is a rectangle of {} square meters", area_struct(&rect1));
    println!("There is a rectangle of {} square meters", rect1.area());
    println!("Rect1 can hold Rect2: {}", rect1.can_hold(&rect2));
    println!("Rect1 can hold Rect3: {}", rect1.can_hold(&rect3));
    println!("Rect2 can hold Rect1: {}", rect2.can_hold(&rect1));
    println!("Rect2 can hold Rect3: {}", rect2.can_hold(&rect3));
    println!("Rect3 can hold Rect1: {}", rect3.can_hold(&rect1));
    println!("Rect3 can hold Rect2: {}", rect3.can_hold(&rect2));
    println!("This is a square: {:?}", Rectangle::square(10));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

