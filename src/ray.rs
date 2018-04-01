use nalgebra::{Point3, Vector3};
use hitable::Hitable;
use sphere::Sphere;
use std::f64;

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

    pub fn color(&self, world: &Hitable) -> Vector3<f64> {
        match world.hit(self, 0.001, f64::INFINITY) {
            Some(hitpoint) => {
                let target = hitpoint.normal + Sphere::random_point_in_unit_sphere();
                0.5 * Ray::new(hitpoint.p, target).color(world)
            }
            None => {
                let unit_direction = unit_vector(&self.direction());
                let t = 0.5 * (unit_direction.y + 1.0);

                let white = Vector3::new(1.0, 1.0, 1.0);
                let cyan = Vector3::new(0.5, 0.7, 1.0);
                (1.0 - t) * white + t * cyan
            }
        }
    }
}

fn unit_vector(v: &Vector3<f64>) -> Vector3<f64> {
    v.clone() / vector_length(&v)
}

fn vector_length(v: &Vector3<f64>) -> f64 {
    v.dot(&v).sqrt()
}
