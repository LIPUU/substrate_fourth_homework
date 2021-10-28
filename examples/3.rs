use std::f32::consts::PI;

fn main() {
    let circle = Circle { radius: 1.2 };

    let square = Square { side_length: 6.6 };
    caculate(circle);
    caculate(square);
}

fn caculate<T>(graphics: T)
where
    T: Areable,
{
    println!("{}", graphics.area());
}

trait Areable {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Areable for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * PI as f64
    }
}

struct Square {
    side_length: f64,
}

impl Areable for Square {
    fn area(&self) -> f64 {
        self.side_length * self.side_length
    }
}
