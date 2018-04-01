use ray::Ray;
use nalgebra::Vector3 as Vector;
use nalgebra::Point3 as Point;
use std::fmt::Debug;

pub struct HitPoint {
    pub t: f32,
    pub p: Point<f32>,
    pub normal: Vector<f32>,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitPoint>;
}

impl<T> Hitable for Vec<T>
where
    T: Hitable + Debug,
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitPoint> {
        self.iter()
            .map(|obj| obj.hit(ray, t_min, t_max))
            .filter(|hitpoint| hitpoint.is_some())
            .map(|hitpoint| hitpoint.unwrap())
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
    }
}
