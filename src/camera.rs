use nalgebra::{Point3, Vector3};
use ray::Ray;
use sphere::Sphere;
use std::f64::consts;
use vec_util;

/// A camera represented as a single 3D point and a rectangular sensor.
/// The sensor is identified by its lower left corner and two directional vectors (one horizontal
/// and one vertical).
///
// /// The sensor is split into a grid of pixels.
#[derive(Copy, Clone)]
pub struct Camera {
    lens: Lens,
    origin: Point3<f64>,
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    w: Vector3<f64>,
}

pub struct Orientation {
    pub look_from: Point3<f64>,
    pub look_at: Point3<f64>,
    pub upwards: Vector3<f64>,
}

#[derive(Copy, Clone)]
pub struct Lens {
    pub aperture: f64,
    pub focal_length: f64,
}

impl Camera {
    /// Creates a new camera centered in `(0, 0, 0)`. The sensor is constructed based on the desired
    /// field of view and aspect ratio.
    pub fn new(orientation: Orientation, lens: Lens, vert_fov: f64, aspect_ratio: f64) -> Camera {
        let theta = vert_fov * consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let w = vec_util::unit(&(orientation.look_from - orientation.look_at));
        let u = vec_util::unit(&(orientation.upwards.cross(&w)));
        let v = w.cross(&u);

        Camera {
            lens,
            origin: orientation.look_from,
            lower_left_corner: (orientation.look_from - lens.focal_length * half_width * u
                - half_height * lens.focal_length * v
                - lens.focal_length * w)
                .coords,
            horizontal: 2.0 * half_width * lens.focal_length * u,
            vertical: 2.0 * half_height * lens.focal_length * v,
            u,
            v,
            w,
        }
    }

    /// Creates a ray which runs from the camera center towards (and through) the screen.
    /// The values `u` and `v` provide the exact direction:
    ///
    /// * Use `(0.0, 0.0)` for obtaining a ray that passes through the lower left corner of the sensor.
    /// * Use `(1.0, 1.0)` for obtaining a ray that passes through the upper right corner of the sensor.
    /// * Use anything in between for obtaining an arbitrary ray.
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius() * Sphere::random_point_in_unit_disk();

        let offset = self.u * rd.x + self.v * rd.y;
        let direction = self.lower_left_corner + u * self.horizontal + v * self.vertical
            - self.origin.coords - offset;
        Ray::new(self.origin.clone() + offset, direction)
    }

    pub fn lens_radius(&self) -> f64 {
        self.lens.aperture / 2.0
    }
}
