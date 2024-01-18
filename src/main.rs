mod boat;
mod simulation;
mod common;

use boat::Boat;
use simulation::Simulation;

// use crate::common::{Pose2D, Velocity2D};

fn main() {
    let mut boat = Boat::new("boat1");
    let mut simulation = Simulation::new(vec![boat]);
    println!("{:?}", simulation);
    simulation.run(100, 0.01);
    println!("{:?}", simulation);
}
