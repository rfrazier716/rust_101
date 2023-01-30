use std::f64::consts::PI;

#[derive(Debug)]
struct EquilateralPolygon {
    n_sides: u32,
    side_length: f64,
}

impl EquilateralPolygon {
    fn from_inscribed_radius(n_sides: u32, radius: f64) -> Self {
        Self {
            n_sides,
            side_length: 2.0 * radius * (PI / (n_sides as f64)).sin(),
        }
    }
    fn area(&self) -> f64 {
        let n_sides = self.n_sides as f64;
        (self.side_length / 2.0).powf(2.0) * n_sides / (PI / n_sides).tan()
    }

    fn to_tuple(self) -> (u32, f64) {
        // this method will consume the struct and move the values into a tuple
        return (self.n_sides, self.side_length);
    }

    fn can_contain(&self, other: &EquilateralPolygon) -> bool {
        self.area() >= other.area()
    }
}

fn main() {
    let rect1 = EquilateralPolygon {
        n_sides: 8,
        side_length: 1.0,
    };
    let rect2 = EquilateralPolygon::from_inscribed_radius(1000, 1.0);
    println!("Rect has an area of: {:?}", rect2.area());
    println!(
        "The Area of the {}-sided polygon is {:.06} Square Pixels",
        rect1.n_sides,
        rect1.area()
    );
    println!("{:?}", rect1.can_contain(&rect2));
}
