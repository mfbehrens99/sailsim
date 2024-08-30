use nalgebra::Vector3;

use crate::boat::consts::*;
use crate::math::{Pose2D, Velocity2D, Wrench2D};

#[derive(Clone, Debug, Default)]
pub struct Boat {
    name: String,
    pose: Vector3<f64>,
    velocity: Vector3<f64>,
}

impl Boat {
    pub fn new(name: impl Into<String>) -> Boat {
        Boat { name: name.into(), pose: Vector3::default(), velocity: Vector3::default() }
    }

    pub fn get_initial(&self) -> Vec<f64> {
        vec![self.pose.x, self.pose.y, self.pose.z, self.velocity.x, self.velocity.y, self.velocity.z]
    }

    pub fn resulting_force(
        &self,
        pos_x: f64,
        pos_y: f64,
        heading: f64,
        vel_x: f64,
        vel_y: f64,
        vel_ang: f64,
    ) -> Wrench2D {
        let pose = Pose2D::new(pos_x, pos_y, heading);
        let velocity = Velocity2D::new(vel_x, vel_y, vel_ang);

        self.get_wrench()
    }

    fn get_wrench(&self) -> Wrench2D {
        Wrench2D::new(1.0, 0.01, 0.0)
    }
}
