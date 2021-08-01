
#[derive(Debug, Copy, Clone)]
struct Point (i32, i32);

#[derive(Debug)]
struct Rectangle {
    top_corner: Point,
    bottom_corner: Point,
}

impl Rectangle {
    fn width(&self) -> u32 {
        let denormalized = self.bottom_corner.0 - self.top_corner.0;
        let normalized = if denormalized < 0 {
            denormalized * -1
        } else {
            denormalized
        };

        normalized as u32
    }

    fn height(&self) -> u32 {
        let denormalized = self.bottom_corner.1 - self.top_corner.1;
        let normalized = if denormalized < 0 {
            denormalized * -1
        } else {
            denormalized
        };

        normalized as u32
    }

    fn area(&self) -> u32 {
        self.width() * self.height()
    }

    fn is_bigger(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }

    fn square(top_corner: Point, side_length: u32) -> Rectangle {
        Rectangle{
            top_corner: top_corner,
            bottom_corner: Point(
                top_corner.0 + side_length as i32, 
                top_corner.1 + side_length as i32,
            ),
        }
    }
}

fn main() {
    let r = Rectangle {
        top_corner: Point(0, 0),
        bottom_corner: Point(10, 10),
    };

    let r2 = Rectangle {
        top_corner: Point(0, 0),
        bottom_corner: Point(5, 5),
    };

    let r3 = Rectangle {
        top_corner: Point(0, 0),
        bottom_corner: Point(30, 5),
    };

    let sq = Rectangle::square(Point(0, 1), 64);
    println!("sq {:?}", sq);

    println!("Hello, world! {}", sq.area());
    println!("he fits {}", r.is_bigger(&r2));
    println!("he fits {}", r.is_bigger(&r3));
}
