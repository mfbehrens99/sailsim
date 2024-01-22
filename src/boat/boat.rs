use crate::common::{Pose2D, Velocity2D, Wrench2D};

#[derive(Clone, Debug, Default)]
pub struct Boat {
    name: String,
    pose: Pose2D,
    velocity: Velocity2D,
}

impl Boat {
    pub fn new(name: &str) -> Boat {
        Boat {
            name: String::from(name),
            pose: Pose2D::default(),
            velocity: Velocity2D::default(),
        }
    }

    pub fn step(&mut self, dt: f64) {
        let wrench = self.get_wrench();
        self.velocity.apply(&wrench, dt);
        self.pose.apply(&self.velocity, dt)
    }

    fn get_wrench(&self) -> Wrench2D {
        Wrench2D::new(1.0, 1.0, 0.0)
    }
}
