use nalgebra::Vector3 as Vector;
use nalgebra::Point3 as Point;
use hitable::Hitable;
use sphere::Sphere;
use std::f32;

#[derive(Debug)]
pub struct Ray {
    origin: Point<f32>,
    direction: Vector<f32>,
}

fn unit_vector(v: &Vector<f32>) -> Vector<f32> {
    v.clone() / vector_length(&v)
}

fn vector_length(v: &Vector<f32>) -> f32 {
    v.dot(&v).sqrt()
}

impl Ray {
    pub fn new(origin: &Point<f32>, direction: &Vector<f32>) -> Ray {
        Ray {
            origin: origin.clone(),
            direction: direction.clone(),
        }
    }

    pub fn origin(&self) -> &Point<f32> {
        &self.origin
    }

    pub fn direction(&self) -> &Vector<f32> {
        &self.direction
    }

    pub fn point_at_parameter(&self, t: f32) -> Point<f32> {
        &self.origin + t * &self.direction
    }

    pub fn color(&self, world: &Hitable) -> Vector<f32> {
        match world.hit(self, 0.0, f32::INFINITY) {
            Some(hitpoint) => {
                let target = hitpoint.normal + Sphere::random_point_in_unit_sphere();

                // 0.5 * (hitpoint.normal + Vector::new(1.0, 1.0, 1.0))
                0.5 * Ray::new(&hitpoint.p, &target).color(world)
            }
            None => {
                let unit_direction = unit_vector(&self.direction());
                let t = 0.5 * (unit_direction.y + 1.0);
                Vector::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector::new(0.5, 0.7, 1.0) * t
            }
        }
    }
}
