use core::fmt;
use fmt::Formatter;
use nalgebra::Vector2;
use std::ops::Add;

#[derive(Clone, Copy, Default)]
pub struct Pose2D {
    point: Vector2<f64>,
    heading: f64,
}

impl Pose2D {
    pub fn new(pos_x: f64, pos_y: f64, heading: f64) -> Pose2D {
        Pose2D {
            point: Vector2::new(pos_x, pos_y),
            heading,
        }
    }

    pub fn apply(&mut self, velocity: &Velocity2D, dt: f64) {
        self.point += velocity.vector * dt;
        self.heading += velocity.angular * dt;
    }
}

#[derive(Clone, Copy, Default)]
pub struct Velocity2D {
    vector: Vector2<f64>,
    angular: f64,
}

impl Velocity2D {
    pub fn new(vel_x: f64, vel_y: f64, vel_ang: f64) -> Velocity2D {
        Velocity2D {
            vector: Vector2::new(vel_x, vel_y),
            angular: vel_ang,
        }
    }

    pub fn apply(&mut self, wrench: &Wrench2D, dt: f64) {
        self.vector += wrench.force * dt;
        self.angular += wrench.torque * dt;
    }
}

#[derive(Clone, Copy, Default)]
pub struct Wrench2D {
    force: Vector2<f64>,
    torque: f64,
}

impl Wrench2D {
    pub fn new(force_x: f64, force_y: f64, torque: f64) -> Wrench2D {
        Wrench2D {
            force: Vector2::new(force_x, force_y),
            torque,
        }
    }
}

impl Into<(f64, f64, f64)> for Wrench2D {
    fn into(self) -> (f64, f64, f64) {
        (self.force[0], self.force[1], self.torque)
    }
}

impl Add for Wrench2D {
    type Output = Wrench2D;
    fn add(self, rhs: Self) -> Self::Output {
        Wrench2D {
            force: self.force + rhs.force,
            torque: self.torque + rhs.torque,
        }
    }
}

// Debug trait implementations
impl fmt::Debug for Pose2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Pose2D")
            .field(&self.point.x)
            .field(&self.point.y)
            .field(&self.heading)
            .finish()
    }
}

impl fmt::Debug for Velocity2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Velocity")
            .field(&self.vector.x)
            .field(&self.vector.y)
            .field(&self.angular)
            .finish()
    }
}

impl fmt::Debug for Wrench2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Wrench2D")
            .field(&self.force.x)
            .field(&self.force.y)
            .field(&self.torque)
            .finish()
    }
}
