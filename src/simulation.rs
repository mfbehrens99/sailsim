use custom_debug_derive::Debug;

use crate::boat::Boat;
use crate::wind::Windfield;

#[derive(Debug, Default)]
pub struct Simulation {
    boats: Vec<Boat>,
    #[debug(skip)]
    windfields: Vec<Box<dyn Windfield>>,
    time: f64,
}

impl Simulation {
    pub fn new(boats: Vec<Boat>, windfields: Vec<Box<dyn Windfield>>) -> Self {
        Self {
            boats,
            windfields,
            time: 0.0,
        }
    }

    pub fn step(&mut self, dt: f64) {
        self.time += dt;
        for boat in &mut self.boats {
            boat.step(dt);
        }
    }

    pub fn run(&mut self, steps: i32, dt: f64) {
        for _step in 0..steps {
            self.step(dt);
        }
    }
}

// impl fmt::Debug for Simulation {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Simulation@{:.3}s\n{}", self.time, self.boats.join("\n"))
//     }
// }
