#[derive(Debug, Clone, PartialEq, Default, Eq, Copy)]
struct V2d<T> {
    x: T,
    y: T,
}

impl<T> V2d<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

use std::ops::Add;

impl<T: Add<Output = T> + Copy> Add for V2d<T> {
    type Output = V2d<T>;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

fn main() {
    let w0 = V2d::new(71, -13);
    let w1 = V2d::new(1, 3);
    let w2 = V2d::new("kot", "pies");
    let w3 = V2d::new(1.1, 3.0);
    let w4 = V2d::new("kot".to_string(), "pies".to_string());

    dbg!(w0);
    dbg!(w1);
    dbg!(w2);
    dbg!(w3);
    dbg!(&w4); // czemu ta referencja jest tu potrzebna?

    dbg!(w0 == w1);
    dbg!(w2 == w2);
    dbg!(w3 == w3);
    dbg!(w4 == w4);

    dbg!(w0 + w1);
    dbg!(w1 + w1);
    dbg!(w3 + w3);
    //     dbg!(w2 + w2);
}