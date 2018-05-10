use nalgebra::{Point3, Vector3};
use ray::Ray;
use std::f64::consts;

/// A camera represented as a single 3D point and a rectangular sensor.
/// The sensor is identified by its lower left corner and two directional vectors (one horizontal
/// and one vertical).
///
// /// The sensor is split into a grid of pixels.
#[derive(Copy, Clone)]
pub struct Camera {
    origin: Point3<f64>,
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
}

impl Camera {
    /// Creates a new camera centered in `(0, 0, 0)`. The sensor is constructed based on the desired
    /// field of view and aspect ratio.
    pub fn new(vert_fov: f64, aspect_ratio: f64) -> Camera {
        let theta = vert_fov * consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        Camera {
            origin: Point3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vector3::new(-half_width, -half_height, -1.0),
            horizontal: Vector3::new(2.0 * half_width, 0.0, 0.0),
            vertical: Vector3::new(0.0, 2.0 * half_height, 0.0),
        }
    }

    /// Creates a ray which runs from the camera center towards (and through) the screen.
    /// The values `u` and `v` provide the exact direction:
    ///
    /// * Use `(0.0, 0.0)` for obtaining a ray that passes through the lower left corner of the sensor.
    /// * Use `(1.0, 1.0)` for obtaining a ray that passes through the upper right corner of the sensor.
    /// * Use anything in between for obtaining an arbitrary ray.
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.lower_left_corner + u * self.horizontal + v * self.vertical;
        Ray::new(self.origin.clone(), direction)
    }
}
