use nalgebra::Vector3;
use std::cmp::Ordering;

pub fn unit(v: &Vector3<f64>) -> Vector3<f64> {
    v.clone() / length(&v)
}

pub fn length(v: &Vector3<f64>) -> f64 {
    v.dot(&v).sqrt()
}

pub fn reflection(v: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(&normal) * normal
}

pub fn refraction(
    v: &Vector3<f64>,
    normal: &Vector3<f64>,
    refractive_index_ratio: f64,
) -> Option<Vector3<f64>> {
    let dt = unit(&v).dot(&normal);
    let discriminant = 1.0 - refractive_index_ratio.powf(2.0) * (1.0 - dt.powf(2.0));

    match discriminant.partial_cmp(&0.0) {
        Some(Ordering::Greater) => {
            let refracted_ray =
                refractive_index_ratio * (unit(&v) - normal * dt) - normal * discriminant.sqrt();
            Some(refracted_ray)
        }
        _ => None,
    }
}

pub fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powf(2.0);

    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
