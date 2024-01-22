use core::fmt;
use fmt::Formatter;
use nalgebra::{Point2, Vector2};

#[derive(Clone, Copy, Default)]
pub struct Pose2D {
    point: Point2<f64>,
    heading: f64,
}

impl Pose2D {
    pub fn new(pos_x: f64, pos_y: f64, heading: f64) -> Pose2D {
        Pose2D {
            point: Point2::new(pos_x, pos_y),
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
        self.vector += wrench.vector * dt;
        self.angular += wrench.torque * dt;
    }
}

#[derive(Clone, Copy, Default)]
pub struct Wrench2D {
    vector: Vector2<f64>,
    torque: f64,
}

impl Wrench2D {
    pub fn new(force_x: f64, force_y: f64, torque: f64) -> Wrench2D {
        Wrench2D {
            vector: Vector2::new(force_x, force_y),
            torque,
        }
    }
}

// Debug trait implementations
impl fmt::Debug for Pose2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.point.x)
            .field(&self.point.y)
            .field(&self.heading)
            .finish()
        // write!(f, "{:.2}|{:.2}:{:.1}°", self.point.x, self.point.y, self.heading / PI * 180.0)
    }
}

impl fmt::Debug for Velocity2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.vector.x)
            .field(&self.vector.y)
            .field(&self.angular)
            .finish()
        // write!(f, "{:.2}|{:.2}:{:.1}°", self.point.x, self.point.y, self.heading / PI * 180.0)
    }
}

impl fmt::Debug for Wrench2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.vector.x)
            .field(&self.vector.y)
            .field(&self.torque)
            .finish()
        // write!(f, "{:.2}|{:.2}:{:.1}°", self.point.x, self.point.y, self.heading / PI * 180.0)
    }
}