use nalgebra::Vector3;
use std::cmp::Ordering;

/// Length of a vector.
pub fn length(v: &Vector3<f64>) -> f64 {
    v.dot(&v).sqrt()
}

/// [Reflection](https://upload.wikimedia.org/wikipedia/commons/1/10/Reflection_angles.svg)
pub fn reflection(v: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
    v - 2. * v.dot(&normal) * normal
}

/// [Refraction](https://en.wikipedia.org/wiki/Refraction#/media/File:RefractionReflextion.svg)
pub fn refraction(
    v: &Vector3<f64>,
    normal: &Vector3<f64>,
    refractive_index_ratio: f64,
) -> Option<Vector3<f64>> {
    let dt = v.normalize().dot(&normal);
    let discriminant = 1. - refractive_index_ratio.powf(2.) * (1. - dt.powf(2.));

    match discriminant.partial_cmp(&0.) {
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
    let r = ((1. - refractive_index) / (1. + refractive_index)).powf(2.);
    r + (1. - r) * (1. - cosine).powf(5.)
}
