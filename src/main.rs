mod boat;
mod common;
mod simulation;
mod wind;

use boat::Boat;
use simulation::Simulation;
use wind::WindfieldStatic;

// use crate::common::{Pose2D, Velocity2D};

fn main() {
    let boat = Boat::new("boat1");
    let wind = WindfieldStatic::new("wind1", 0.0, 0.0);
    let mut simulation = Simulation::new(vec![boat], vec![Box::new(wind)]);
    println!("{:.2?}", simulation);
    simulation.run(100, 0.01);
    println!("{:.2?}", simulation);
}
