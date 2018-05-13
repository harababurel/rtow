use hitable::{HitPoint, Hitable};
use material::Material;
use nalgebra::{Point3, Vector3};
use rand::{thread_rng, Rng};
use ray::Ray;
use std::cmp::Ordering;

/// A 3D sphere.
#[derive(Clone, Debug)]
pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn center(&self) -> &Point3<f64> {
        &self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Returns a random 3D Point situated inside a sphere of radius 1, located in the origin.
    /// The point is obtained by sequentially generating points in the unit square and selecting
    /// the first one that happens to also be inside the unit sphere. Approximately 52.35% chance of
    /// getting a valid point on each trial.
    pub fn random_point_in_unit_sphere() -> Vector3<f64> {
        let mut rng = thread_rng();
        (0..)
            .into_iter()
            .map(|_| {
                Vector3::new(
                    rng.gen_range(-1.0, 1.0),
                    rng.gen_range(-1.0, 1.0),
                    rng.gen_range(-1.0, 1.0),
                )
            })
            .filter(|point| point.dot(&point) < 1.0)
            .take(1)
            .next()
            .unwrap()
    }
}

impl Hitable for Sphere {
    /// There can be 0, 1 or 2 hitpoints for a given ray and a sphere. If there is more than
    /// one hitpoint, the closest one (smallest `t`) is chosen.
    fn hit(&self, ray: &Ray, t_min: f64, tmax: f64) -> Option<HitPoint> {
        let oc = ray.origin() - self.center();

        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius().powf(2.0);

        let delta = b.powf(2.0) - a * c;

        match delta.partial_cmp(&0.0) {
            Some(Ordering::Greater) => {
                for t in vec![(-b - delta.sqrt()) / a, (-b + delta.sqrt()) / a] {
                    if t_min < t && t < tmax {
                        let p = ray.point_at_parameter(t);
                        let normal = (p - self.center()) / self.radius();
                        return Some(HitPoint {
                            t,
                            p,
                            normal,
                            material: self.material.clone(),
                        });
                    }
                }
                None
            }
            _ => None,
        }
    }
}
