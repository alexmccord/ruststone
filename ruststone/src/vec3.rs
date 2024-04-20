use std::{
    fmt::Display,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Vec3(pub i32, pub i32, pub i32);

/// We'd like to copy Minecraft's coordinate system so that it's trivial to
/// put in the coordinates or create software that interacts with ruststone.
///
/// A fresh reminder on Minecraft's coordinate system:
///
/// Heading up means to increment the Y-axis, and down to decrement the Y-axis.
/// Heading west means to decrement the X-axis, and east to increment the X-axis.
/// Heading north means to decrement the Z-axis, and south to increment the Z-axis.
impl Vec3 {
    pub fn x(self) -> i32 {
        self.0
    }

    pub fn y(self) -> i32 {
        self.1
    }

    pub fn z(self) -> i32 {
        self.2
    }

    pub fn abs(self) -> Vec3 {
        Vec3(self.x().abs(), self.y().abs(), self.z().abs())
    }

    pub fn up(self) -> Vec3 {
        self + Vec3(0, 1, 0)
    }

    pub fn down(self) -> Vec3 {
        self - Vec3(0, 1, 0)
    }

    pub fn west(self) -> Vec3 {
        self - Vec3(1, 0, 0)
    }

    pub fn east(self) -> Vec3 {
        self + Vec3(1, 0, 0)
    }

    pub fn north(self) -> Vec3 {
        self - Vec3(0, 0, 1)
    }

    pub fn south(self) -> Vec3 {
        self + Vec3(0, 0, 1)
    }
}

impl From<(i32, i32, i32)> for Vec3 {
    fn from((x, y, z): (i32, i32, i32)) -> Vec3 {
        Vec3(x, y, z)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, Vec3(x, y, z): Vec3) -> Vec3 {
        Vec3(self.x() + x, self.y() + y, self.z() + z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, Vec3(x, y, z): Vec3) -> Vec3 {
        Vec3(self.x() - x, self.y() - y, self.z() - z)
    }
}
