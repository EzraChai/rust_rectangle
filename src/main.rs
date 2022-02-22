#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        &self.width * &self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        &self.width >= &other.width && &self.height >= &other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let s = Rectangle::square(23);
    println!("{:#?}", s);

    let rectangle1 = Rectangle {
        width: 23,
        height: 34,
    };
    let rectangle2 = Rectangle {
        width: 232,
        height: 43,
    };
    let rectangle3 = Rectangle {
        width: 11,
        height: 22,
    };

    println!(
        "Width: {}, Heigth: {}, Area: {}",
        rectangle1.width,
        rectangle1.height,
        rectangle1.area()
    );

    println!("{}", rectangle1.can_hold(&rectangle3));

    println!("{:#?}", rectangle1);
}
