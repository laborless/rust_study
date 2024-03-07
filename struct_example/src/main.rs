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
        self.width > other.width && self.height > other.height
    }
}
impl Rectangle {
    fn square(size: u32) -> Self {
        Self { 
            width: size,
            height: size
        }
    }
}

fn main() {
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
    let rect4 = Rectangle::square(20);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    dbg!(&rect4);
    // let scale = 2;
    // let width1 = 30;
    // let height1 = 50;
    // let rect1 = (30, 50);
    // let rect1 = Rectangle {
    //     width:dbg!(30 * scale),
    //     height: 50,
    // };

    // println!("rect1 is {:#?}", rect1);
    // println!(
    //     "The area of the rectangle is {} square pixels.", 
    //     // area(width1, height1)
    //     // area(&rect1)
    //     rect1.area()
    // );
    // dbg!(&rect1);

    // if rect1.width() {
    //     println!("The rectangle has a nonzero width; it is {}", rect1.width);
    // }
}

// fn area(rectangle: & Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
// fn area (dimension: (u32, u32)) -> u32 {
//     dimension.0 * dimension.1
// }
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }