use nalgebra::Vector3 as Vector;
use nalgebra::Point3 as Point;
use ray::Ray;

pub struct Camera {
    pub origin: Point<f32>,
    pub lower_left_corner: Vector<f32>,
    pub horizontal: Vector<f32>,
    pub vertical: Vector<f32>,
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let direction = self.lower_left_corner + u * self.horizontal + v * self.vertical;
        Ray::new(&self.origin, &direction)
    }
}
