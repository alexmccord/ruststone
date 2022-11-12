use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Vec3(pub i32, pub i32, pub i32);

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
}

impl From<(i32, i32, i32)> for Vec3 {
    fn from((x, y, z): (i32, i32, i32)) -> Vec3 {
        Vec3(x, y, z)
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
