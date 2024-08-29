use peroxide::fuga::*;
use std::f64::consts::PI;

use crate::boat::consts::{MASS, MOMENT_INTERTIA};
use crate::boat::Boat;
use crate::wind::Windfield;

// #[derive(Debug, Default)]
pub struct Simulation {
    solver: BasicODESolver<RKF45>,
    pub boat: Boat,
    windfields: Vec<Box<dyn Windfield>>,
    time: f64,
}

impl Simulation {
    pub fn new(boat: Boat, windfields: Vec<Box<dyn Windfield>>) -> Self {
        let rkf45 = RKF45::new(1e-4, 0.9, 1e-6, 1e-2, 100);
        let solver = BasicODESolver::new(rkf45);
        Self {
            solver,
            boat,
            windfields,
            time: 0.0,
        }
    }

    pub fn run(&mut self, stop_time: f64, dt: f64) {
        let (t_vec, y_vec) = self
            .solver
            .solve(&self.boat, (self.time, stop_time), dt)
            .unwrap(); // Error handling with `?` - can check constraint violation and etc.

        self.time = stop_time;
        let y_mat = py_matrix(y_vec);
    }
}

impl ODEProblem for Boat {
    fn initial_conditions(&self) -> Vec<f64> {
        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
    }

    fn rhs(&self, _t: f64, y: &[f64], dy: &mut [f64]) -> anyhow::Result<()> {
        let pos_x = y[0]; // x position
        let pos_y = y[1]; // y position
        let heading = y[2]; // orientation angle
        let vel_x = y[3]; // velocity in x
        let vel_y = y[4]; // velocity in y
        let vel_ang = y[5]; // angular velocity

        let (force_x, force_y, torque) = self.resulting_force(pos_x, pos_y, heading, vel_x, vel_y, vel_ang).into();
        (dy[0], dy[1], dy[2]) = (vel_x, vel_y, heading);
        (dy[3], dy[4], dy[5]) = (force_x / MASS, force_y / MASS, torque / MOMENT_INTERTIA);

        Ok(())
    }
}
