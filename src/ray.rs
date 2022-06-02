use crate::hitable::Hitable;
use crate::material::Scatterable;
use nalgebra::{Point3, Vector3};
use std::f64;

/// A ray of light.
#[derive(Debug)]
pub struct Ray {
    /// The source of the ray. Keep in mind that the ray is "reversed" i.e. it starts in the
    /// observer point and goes backwards towards the light emitting source.
    origin: Point3<f64>,
    /// The 3D direction of the ray.
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

    /// The point at some parameter `t` is defined as a 3D point located on the ray at a distance
    /// of `t * direction` from the origin.
    pub fn point_at_parameter(&self, t: f64) -> Point3<f64> {
        &self.origin + t * &self.direction
    }

    /// Traces the ray backwards and computes its color. It simulates at most 100 hit points with
    /// the elements of the world. For each hit point, it continues the simulation using the scattered ray instead of the original one. Depending on the attenuation of the materials which are hit, each consecutive ray loses some color intensity. When no additional object is hit, the world background (a vertical gradient from cyan to white) is used for the color.
    /// Note: when computing hitpoints, `t_min = 0.001` is used in order to prevent [shadow
    /// acne](https://computergraphics.stackexchange.com/questions/2192/cause-of-shadow-acne).
    pub fn color(&self, world: &dyn Hitable, depth: i32) -> Vector3<f64> {
        match world.hit(self, 0.001, f64::INFINITY) {
            Some(hitpoint) => {
                if depth > 100 {
                    return Vector3::new(0., 0., 0.);
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
                    None => Vector3::new(0., 0., 0.),
                }
            }
            None => {
                let unit_direction = self.direction().normalize();
                let t = 0.5 * (unit_direction.y + 1.);

                let white = Vector3::new(1., 1., 1.);
                let cyan = Vector3::new(0.5, 0.7, 1.);
                (1. - t) * white + t * cyan
            }
        }
    }
}
