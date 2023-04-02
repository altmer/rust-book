#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 40,
    };
    println!("Rectangle is {:?}.", rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    println!(
        "The field width is {} and the method width() is {}",
        rect.width,
        rect.width()
    );

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(scale * 30),
        height: 50,
    };

    dbg!(&rect1);

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

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(30);
    println!("Suqare is {:?}", square);
}
