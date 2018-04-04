use nalgebra::{Point3, Vector3};
use hitable::Hitable;
use material::Scatterable;
use std::f64;
use vec_util;

#[derive(Debug)]
pub struct Ray {
    origin: Point3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Point3<f64> {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3<f64> {
        &self.direction
    }

    pub fn point_at_parameter(&self, t: f64) -> Point3<f64> {
        &self.origin + t * &self.direction
    }

    pub fn color(&self, world: &Hitable, depth: i32) -> Vector3<f64> {
        match world.hit(self, 0.001, f64::INFINITY) {
            Some(hitpoint) => {
                if depth > 50 {
                    return Vector3::new(0.0, 0.0, 0.0);
                }

                match hitpoint.material.scatter(self, &hitpoint) {
                    Some((scattered_ray, attenuation)) => {
                        let color = scattered_ray.color(world, depth + 1);

                        Vector3::new(
                            attenuation.x * color.x,
                            attenuation.y * color.y,
                            attenuation.z * color.z,
                        )
                    }
                    None => Vector3::new(0.0, 0.0, 0.0),
                }
            }
            None => {
                let unit_direction = vec_util::unit(&self.direction());
                let t = 0.5 * (unit_direction.y + 1.0);

                let white = Vector3::new(1.0, 1.0, 1.0);
                let cyan = Vector3::new(0.5, 0.7, 1.0);
                (1.0 - t) * white + t * cyan
            }
        }
    }
}
