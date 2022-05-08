#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    let rect1 = Rectangle {
        width: 10,
        height: 20
    };

    let rect2 = Rectangle {
        width: 20,
        height: 30
    };

    let rect3 = Rectangle::square(20);
    
    println!("rect: {:?}", rect);
    println!("the area is {}", rect.area());

    println("rect can hold rect1: {}", rect.can_hold(&rect1));
    println("rect can hold rect2: {}", rect.can_hold(&rect2));
}