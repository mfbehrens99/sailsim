mod boat;
mod math;
mod simulation;
mod wind;

use boat::Boat;
use nalgebra::Vector2;
use peroxide::fuga::RKF45;
use simulation::Simulation;
use wind::StaticWind;

// use crate::common::{Pose2D, Velocity2D};

fn main() {
    let boat = Boat::new("boat1");
    let wind = StaticWind::new(Vector2::new(1.0, 2.));
    let rkf45 = RKF45::new(1e-4, 0.9, 1e-6, 1e-2, 100);
    let mut simulation = Simulation::new(boat, wind, rkf45);
    println!("{:.2?}", simulation.boat);
    simulation.run(100.0, 1e-3);
    println!("{:.2?}", simulation.boat);
}
