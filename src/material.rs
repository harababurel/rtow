use ray::Ray;
use rand::{thread_rng, Rng};
use sphere::Sphere;
use hitable::HitPoint;
use nalgebra::Vector3;
use vec_util;
use std::cmp::Ordering;

type Attenuation = Vector3<f64>;
type Fuzziness = f64;
type RefractiveIndex = f64;

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian(Attenuation),
    Metal(Attenuation, Fuzziness),
    Dielectric(RefractiveIndex),
}

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hitpoint: &HitPoint) -> Option<(Ray, Attenuation)>;
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hitpoint: &HitPoint) -> Option<(Ray, Attenuation)> {
        match self {
            &Material::Lambertian(attenuation) => {
                let direction = hitpoint.normal + Sphere::random_point_in_unit_sphere();
                let scattered_ray = Ray::new(hitpoint.p, direction);
                Some((scattered_ray, attenuation.clone()))
            }
            &Material::Metal(attenuation, fuzziness) => {
                let reflection_direction =
                    vec_util::reflection(&vec_util::unit(ray.direction()), &hitpoint.normal)
                        + fuzziness * Sphere::random_point_in_unit_sphere();
                let scattered_ray = Ray::new(hitpoint.p, reflection_direction);

                match scattered_ray
                    .direction()
                    .dot(&hitpoint.normal)
                    .partial_cmp(&0.0)
                {
                    Some(Ordering::Greater) => Some((scattered_ray, attenuation.clone())),
                    _ => None,
                }
            }
            &Material::Dielectric(refractive_index) => {
                let reflected_vector = vec_util::reflection(&ray.direction(), &hitpoint.normal);
                let air_refractive_index = 1.0;
                let attenuation = Vector3::new(1.0, 1.0, 1.0); // glass absorbs nothing

                let mut outward_normal = hitpoint.normal;
                let mut refractive_index_ratio = air_refractive_index / refractive_index;
                let mut cosine =
                    -ray.direction().dot(&hitpoint.normal) / vec_util::length(ray.direction());

                if ray.direction().dot(&hitpoint.normal) > 0.0 {
                    outward_normal = -hitpoint.normal;
                    refractive_index_ratio = refractive_index / air_refractive_index;
                    cosine = refractive_index * ray.direction().dot(&hitpoint.normal)
                        / vec_util::length(ray.direction());
                }

                let mut final_vector =
                    vec_util::refraction(&ray.direction(), &outward_normal, refractive_index_ratio)
                        .unwrap_or(reflected_vector);

                let reflection_prob = vec_util::schlick(cosine, refractive_index);
                let mut rng = thread_rng();

                if rng.gen_range(0.0, 1.0) < reflection_prob {
                    final_vector = reflected_vector;
                }

                Some((Ray::new(hitpoint.p, final_vector), attenuation))
            }
        }
    }
}
