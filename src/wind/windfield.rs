use crate::wind::Wind;

pub trait Windfield: std::fmt::Debug {
    fn at(&self, x: f64, y: f64) -> Wind;
}

#[derive(Debug, Default)]
pub struct WindfieldStatic {
    name: String,
    wind: Wind,
}

impl WindfieldStatic {
    pub fn new(name: &str, x: f64, y: f64) -> WindfieldStatic {
        WindfieldStatic {
            name: String::from(name),
            wind: Wind::new(x, y),
        }
    }
}

impl Windfield for WindfieldStatic {
    fn at(&self, _x: f64, _y: f64) -> Wind {
        self.wind
    }
}
