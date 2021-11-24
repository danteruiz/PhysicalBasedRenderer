use std::cmp::PartialEq;
use std::convert::From;
use std::fmt;
use std::ops::Mul;

use crate::math::ops::Cross;
use crate::math::vec3::Vec3;

#[derive(Debug)]
pub struct Quat {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Quat {
    // fn new(w: f32, x: f32, y: f32, z: f32) -> Quat {
    //     Quat {
    //         w: w,
    //         x: x,
    //         y: y,
    //         z: z,
    //     }
    // }

    // pub fn identity() -> Quat {
    //     Quat {
    //         w: 1.0,
    //         x: 0.0,
    //         y: 0.0,
    //         z: 0.0,
    //     }
    // }
}

impl Mul for Quat {
    type Output = Self;
    fn mul(self, q: Quat) -> Self {
        Quat {
            w: self.w * q.w - self.x * q.x - self.y * q.y - self.z * q.z,
            x: self.w * q.x + self.x * q.w + self.y * q.z - self.z * q.y,
            y: self.w * q.y + self.y * q.w + self.z * q.x - self.x * q.z,
            z: self.w * q.z + self.z * q.w + self.x * q.y - self.y * q.x,
        }
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        let w = Vec3::new(self.x, self.y, self.z);
        let wx = Vec3::cross(&w, &v);
        let wxx = Vec3::cross(&w, &wx);

        v + ((self.w * wx) + wxx) * 2.0
    }
}

impl From<Vec3> for Quat {
    fn from(v: Vec3) -> Quat {
        let c = (v * 0.5).cos();
        let s = (v * 0.5).sin();

        Quat {
            w: c.x * c.y * c.z + s.x * s.y * s.z,
            x: s.x * c.y * c.z - c.x * s.y * s.z,
            y: c.x * s.y * c.z + s.x * c.y * s.z,
            z: c.x * c.y * s.z - s.x * s.y * c.z,
        }
    }
}

impl PartialEq for Quat {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w && self.x == other.w && self.y == other.y && self.z == other.z
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

impl fmt::Display for Quat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:.6}, {:.6}, {:.6}, {:.6})",
            self.w, self.x, self.y, self.z,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul() {
        let q = Quat::from(Vec3::new(0.0, 60.0, 190.0).to_radians());
        let q1 = Quat::from(Vec3::new(10.0, -40.0, 50.0).to_radians());

        let result = Quat::new(-0.330961, -0.214658, 0.383051, 0.83526);

        assert_eq!(q * q1, result);
    }
    #[test]
    fn from_vec3() {
        let q = Quat::from(Vec3::new(10.0, -40.0, 50.0).to_radians());
        let result = Quat::new(0.835812, 0.21822, -0.274184, 0.422636);

        assert_eq!(q, result);
    }
}
