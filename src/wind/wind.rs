use std::ops::Add;
use nalgebra::Vector2;

// use wind::Windfield;

#[derive(Clone, Copy, Debug, Default)]
pub struct Wind {
    vector: Vector2<f64>,
}

impl Wind {
    pub fn new (x: f64, y: f64) -> Wind {
        Wind {
            vector: Vector2::new(x,y),
        }
    }
}

impl Add for Wind {
    type Output = Wind;

    fn add(self, other: Wind) -> Wind {
        Wind {
            vector: self.vector + other.vector,
        }
    }
}
