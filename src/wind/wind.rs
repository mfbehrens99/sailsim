use nalgebra::Vector2;
use std::ops::Add;

pub trait Wind {
    fn at(&self, x: f64, y: f64, t: f64) -> Vector2<f64>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StaticWind {
    vector: Vector2<f64>,
}

impl StaticWind {
    pub fn new(vector: Vector2<f64>) -> StaticWind {
        StaticWind { vector }
    }
}

impl Add for StaticWind {
    type Output = StaticWind;

    fn add(self, other: StaticWind) -> StaticWind {
        StaticWind {
            vector: self.vector + other.vector,
        }
    }
}

impl Wind for StaticWind {
    fn at(&self, _x: f64, _y: f64, _t: f64) -> Vector2<f64> {
        self.vector
    }
}
