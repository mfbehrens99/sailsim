// #[derive(Clone, Copy, Default, Debug)]
// pub struct Pose2D {
//     x: f64,
//     y: f64,
//     heading: f64,
// }

// impl Pose2D {
//     fn apply(mut self, velocity: &Velocity2D, dt: f64) {
//         self.x += velocity.x * dt;
//         self.y += velocity.y * dt;
//         self.heading += velocity.angular * dt;
//     }
// }
// #[derive(Clone, Copy, Default, Debug)]
// pub struct Velocity2D {
//     x: f64,
//     y: f64,
//     angular: f64,
// }

// impl Velocity2D {
//     fn apply(mut self, wrench: &Wrench2D, dt: f64) {
//         self.x += wrench.x * dt;
//         self.y += wrench.y * dt;
//         self.angular += wrench.torque * dt;
//     }
// }

// #[derive(Clone, Copy, Default, Debug)]
// pub struct Wrench2D {
//     x: f64,
//     y: f64,
//     torque: f64,
// }

use core::fmt;
use fmt::Formatter;
use std::f64::consts::PI;
use nalgebra::{Vector2, Point2};

#[derive(Clone, Copy, Default)]
pub struct Pose2D {
    point: Point2<f64>,
    heading: f64,
}

impl Pose2D {
    pub fn apply(&mut self, velocity: &Velocity2D, dt: f64) {
        self.point += velocity.vector * dt;
        self.heading += velocity.angular * dt;
    }
}

impl fmt::Debug for Pose2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}|{:.2}:{:.1}°", self.point.x, self.point.y, self.heading / PI * 180.0)
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Velocity2D {
    vector: Vector2<f64>,
    angular: f64,
}

impl Velocity2D {
    pub fn apply(&mut self, wrench: &Wrench2D, dt: f64) {
        self.vector += wrench.vector * dt;
        self.angular += wrench.torque * dt;
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Wrench2D {
    vector: Vector2<f64>,
    torque: f64,
}

impl Wrench2D {
    pub fn new(x: f64, y: f64, torque: f64) -> Wrench2D{
        Wrench2D {
            vector: Vector2::new(x,y),
            torque: torque,
        }
    }
}