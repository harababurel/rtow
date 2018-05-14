use nalgebra::Vector3;
use std::cmp::Ordering;

/// Length of a vector.
pub fn length(v: &Vector3<f64>) -> f64 {
    v.dot(&v).sqrt()
}

/// [Reflection](https://upload.wikimedia.org/wikipedia/commons/1/10/Reflection_angles.svg)
pub fn reflection(v: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(&normal) * normal
}

/// [Refraction](https://en.wikipedia.org/wiki/Refraction#/media/File:RefractionReflextion.svg)
pub fn refraction(
    v: &Vector3<f64>,
    normal: &Vector3<f64>,
    refractive_index_ratio: f64,
) -> Option<Vector3<f64>> {
    let dt = v.normalize().dot(&normal);
    let discriminant = 1.0 - refractive_index_ratio.powf(2.0) * (1.0 - dt.powf(2.0));

    match discriminant.partial_cmp(&0.0) {
        Some(Ordering::Greater) => {
            let refracted_ray = refractive_index_ratio * (v.normalize() - normal * dt)
                - normal * discriminant.sqrt();
            Some(refracted_ray)
        }
        _ => None,
    }
}

/// [Schlick's approximation](https://en.wikipedia.org/wiki/Schlick%27s_approximation)
pub fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let r = ((1.0 - refractive_index) / (1.0 + refractive_index)).powf(2.0);
    r + (1.0 - r) * (1.0 - cosine).powf(5.0)
}
