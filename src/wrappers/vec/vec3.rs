use std::ops::{AddAssign, SubAssign};

use nalgebra::{Isometry3, Unit, Vector3 as NVector3};
use parry3d::math::Point;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Vector3 {
    pub inner: NVector3<f32>,
}

impl Vector3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            inner: NVector3::new(x, y, z),
        }
    }

    pub fn x(&self) -> f32 {
        self.inner.x
    }

    pub fn y(&self) -> f32 {
        self.inner.y
    }

    pub fn z(&self) -> f32 {
        self.inner.z
    }

    pub fn add(&mut self, other: Vector3) -> Vector3 {
        self.inner.add_assign(other.inner);
        *self
    }

    pub fn sub(&mut self, other: Vector3) -> Vector3 {
        self.inner.sub_assign(other.inner);
        *self
    }
}

impl Into<NVector3<f32>> for Vector3 {
    fn into(self) -> NVector3<f32> {
        self.inner
    }
}

impl Into<Unit<NVector3<f32>>> for Vector3 {
    fn into(self) -> Unit<NVector3<f32>> {
        Unit::new_normalize(self.inner)
    }
}

impl Into<Isometry3<f32>> for Vector3 {
    fn into(self) -> Isometry3<f32> {
        self.inner.into()
    }
}

impl Into<Point<f32>> for Vector3 {
    fn into(self) -> Point<f32> {
        Point::new(self.inner.x, self.inner.y, self.inner.z)
    }
}
