
#[derive(Debug)]
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
}

fn main() {
    let r = Rectangle{
        top_corner: Point(0, 0),
        bottom_corner: Point(10, 10)
    };
    println!("Hello, world! {}", r.area());
}
