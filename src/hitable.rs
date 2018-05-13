use material::Material;
use nalgebra::{Point3, Vector3};
use ray::Ray;
use std::fmt::Debug;

/// The point of contact between a ray and a material.
pub struct HitPoint {
    /// the distance along the ray where the contact happens; more specifically, it satisfies the
    /// equation `ray.origin + t * ray.direction() = p`.
    pub t: f64,
    /// the actual 3D contact point.
    pub p: Point3<f64>,
    /// the normal vector on the material surface in point `p`.
    pub normal: Vector3<f64>,
    /// describes material properties; useful for determining what happens to the ray
    /// after contact (is it absorbed/reflected/refracted?).
    pub material: Material,
}

/// A `Hitable` object is anything that can be hit by a `Ray`, resulting in a `HitPoint`.
pub trait Hitable {
    /// Returns the `HitPoint` (if any) of a given `Ray` that hits the object at a `t` in `[t_min,
    /// t_max]`. If there are multiple such hit points, the closest one (smallest `t`) is used.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitPoint>;
}

impl<T> Hitable for Vec<T>
where
    T: Hitable + Debug,
{
    /// Returns the closest hitpoint (smallest `t`) of all `Hitable` objects contained in the
    /// `Vec`.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitPoint> {
        self.iter()
            .map(|obj| obj.hit(ray, t_min, t_max))
            .filter(|hitpoint| hitpoint.is_some())
            .map(|hitpoint| hitpoint.unwrap())
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
    }
}
