use nalgebra::Point3 as Point;
use nalgebra::Vector3 as Vector;
use std::cmp::Ordering;
use hitable::{HitPoint, Hitable};
use ray::Ray;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Sphere {
    center: Point<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(center: &Point<f32>, radius: f32) -> Sphere {
        Sphere {
            center: center.clone(),
            radius,
        }
    }

    pub fn center(&self) -> &Point<f32> {
        &self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn random_point_in_unit_sphere() -> Vector<f32> {
        let mut rng = thread_rng();
        (0..)
            .into_iter()
            .map(|_| {
                Vector::new(
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
    fn hit(&self, ray: &Ray, t_min: f32, tmax: f32) -> Option<HitPoint> {
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
                        return Some(HitPoint { t, p, normal });
                    }
                }
                None
            }
            _ => None,
        }
    }
}
