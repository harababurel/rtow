use nalgebra::Vector3 as Vector;
use std::ops::Mul;
use std::cmp;

pub struct Pixel {
    pub r: u16,
    pub g: u16,
    pub b: u16,
}

impl Pixel {
    pub fn from(v: &Vector<f32>) -> Self {
        Pixel {
            r: v.x as u16,
            g: v.y as u16,
            b: v.z as u16,
        }
    }
}

impl Mul<f32> for Pixel {
    type Output = Pixel;

    fn mul(self, scalar: f32) -> Pixel {
        Pixel {
            r: cmp::max(0, cmp::min(255, (self.r as f32 * scalar) as i32)) as u16,
            g: cmp::max(0, cmp::min(255, (self.g as f32 * scalar) as i32)) as u16,
            b: cmp::max(0, cmp::min(255, (self.b as f32 * scalar) as i32)) as u16,
        }
    }
}
