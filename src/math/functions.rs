use std::ops::{Mul, Sub};

use nalgebra::Vector2;

pub fn cross_2d<T, U, V>(lhs: Vector2<T>, rhs: Vector2<T>) -> V
where
    T: Copy + Mul<Output = U>,
    U: Sub<Output = V>,
{
    lhs[0] * rhs[1] - lhs[1] * rhs[0]
}
