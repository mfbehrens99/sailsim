use core::fmt;

use crate::boat::Boat;
#[derive(Debug, Default)]
pub struct Simulation {
    boats: Vec<Boat>,
    time: f64,
}

impl Simulation {
    pub fn new(boats: Vec<Boat>) -> Self {
        Self {
            boats,
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
