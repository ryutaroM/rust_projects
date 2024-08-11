fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_v2(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_v3(&rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!("the rect1 is {}", rect1);
    println!("the rect1 is {:?}", rect1);
    println!("the rect1 is {:#?}", rect1);

    let rect1 = RectangleV2 {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rec1 = RectangleV2 {
        width: 30,
        height: 50,
    };
    let rec2 = RectangleV2 {
        width: 10,
        height: 40,
    };
    let rec3 = RectangleV2 {
        width: 60,
        height: 45,
    };

    println!("Can rec1 hold rec2? {}", rec1.can_hold(&rec2));
    println!("Can rec1 hold rec3? {}", rec1.can_hold(&rec3));

    let sq = RectangleV2::square(50);
    println!("square {:#?}", sq);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_v2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_v3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct RectangleV2 {
    width: u32,
    height: u32,
}

impl RectangleV2 {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

//multi impl block
impl RectangleV2 {
    fn can_hold(&self, other: &RectangleV2) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> RectangleV2 {
        RectangleV2 {
            width: size,
            height: size,
        }
    }
}
