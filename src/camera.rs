use nalgebra::{Point3, Vector3};
use ray::Ray;

pub struct Camera {
    pub origin: Point3<f64>,
    pub lower_left_corner: Vector3<f64>,
    pub horizontal: Vector3<f64>,
    pub vertical: Vector3<f64>,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.lower_left_corner + u * self.horizontal + v * self.vertical;
        Ray::new(self.origin.clone(), direction)
    }
}
