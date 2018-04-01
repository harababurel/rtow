use ray::Ray;
use nalgebra::{Point3, Vector3};
use std::fmt::Debug;

pub struct HitPoint {
    pub t: f64,
    pub p: Point3<f64>,
    pub normal: Vector3<f64>,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitPoint>;
}

impl<T> Hitable for Vec<T>
where
    T: Hitable + Debug,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitPoint> {
        self.iter()
            .map(|obj| obj.hit(ray, t_min, t_max))
            .filter(|hitpoint| hitpoint.is_some())
            .map(|hitpoint| hitpoint.unwrap())
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
    }
}
