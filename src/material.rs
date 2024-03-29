use crate::hitable::HitPoint;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::util;
use nalgebra::Vector3;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

/// Percentage of each RGB color that persists after a ray gets scattered.
pub type Attenuation = Vector3<f64>;

/// `0.0` for smooth and shiny; `1.0` for fuzzy.
pub type Fuzziness = f64;

/// [Refractive index](https://en.wikipedia.org/wiki/Refractive_index)
pub type RefractiveIndex = f64;

#[derive(Debug, Copy, Clone)]
pub enum Material {
    /// Matte.
    Lambertian { attenuation: Vector3<f64> },
    /// Metal.
    Metal {
        attenuation: Vector3<f64>,
        fuzziness: f64,
    },
    /// i.e. glass.
    Dielectric { refractive_index: f64 },
}

impl Material {
    pub fn random_lambertian() -> Material {
        let mut rng = thread_rng();
        Material::Lambertian {
            attenuation: Vector3::new(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            ),
        }
    }

    pub fn random_metal() -> Material {
        let mut rng = thread_rng();
        Material::Metal {
            attenuation: Vector3::new(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            ),
            fuzziness: rng.gen_range(0.0..1.0),
        }
    }

    pub fn random_dielectric() -> Material {
        let mut rng = thread_rng();
        Material::Dielectric {
            refractive_index: rng.gen_range(1.3..3.2),
        }
    }

    /// Box the material-generating closures in order to make them lazy.
    /// This way only one material is generated.
    pub fn random_material() -> Material {
        let mut rng = thread_rng();
        let fns = vec![
            Box::new(Material::random_metal) as Box<dyn Fn() -> Material>,
            Box::new(Material::random_lambertian) as Box<dyn Fn() -> Material>,
            Box::new(Material::random_dielectric) as Box<dyn Fn() -> Material>,
        ];
        fns[rng.gen::<usize>() % 3]()
    }
}

/// A `Scatterable` scatters the light rays that hit it.
pub trait Scatterable {
    /// Returns the new `Ray` (if any) and its `Attenuation` which results from a given `Ray` hitting a `Scatterable` object.
    /// The new ray usually describes a physical phenomenon (reflection, refraction or absorption).
    fn scatter(&self, ray: &Ray, hitpoint: &HitPoint) -> Option<(Ray, Attenuation)>;
}

impl Scatterable for Material {
    /// Different materials scatter in different ways:
    ///
    /// * A `Lambertian` (matte) object reflects the ray along the direction of the normal vector,
    /// which is slightly altered by adding a random delta.
    /// * A `Metal` reflects the ray along a direction which is [symmetrical to the normal
    /// vector](https://upload.wikimedia.org/wikipedia/commons/1/10/Reflection_angles.svg).
    /// Depending on the fuzziness of the metal, a random delta may be added to this direction.
    /// * A `Dielectric` (i.e. glass) material can either reflect or refract the ray. The
    /// probability of each event depends on multiple factors, such as the refractive index and the
    /// angle of incidence. This probability is roughly approximated by the `schlick` polynomial.
    fn scatter(&self, ray: &Ray, hitpoint: &HitPoint) -> Option<(Ray, Attenuation)> {
        match *self {
            Material::Lambertian { attenuation } => {
                let direction =
                    hitpoint.normal.into_inner() + Sphere::random_point_in_unit_sphere();
                let scattered_ray = Ray::new(hitpoint.p, direction);
                Some((scattered_ray, attenuation))
            }
            Material::Metal {
                attenuation,
                fuzziness,
            } => {
                let reflection_direction =
                    util::reflection(&ray.direction().normalize(), &hitpoint.normal)
                        + fuzziness * Sphere::random_point_in_unit_sphere();
                let scattered_ray = Ray::new(hitpoint.p, reflection_direction);

                match scattered_ray
                    .direction()
                    .dot(&hitpoint.normal)
                    .partial_cmp(&0.)
                {
                    Some(Ordering::Greater) => Some((scattered_ray, attenuation)),
                    _ => None,
                }
            }
            Material::Dielectric { refractive_index } => {
                let reflected_vector = util::reflection(ray.direction(), &hitpoint.normal);
                let air_refractive_index = 1.;
                let attenuation = Vector3::new(1., 1., 1.); // glass absorbs nothing

                let mut outward_normal = hitpoint.normal;
                let mut refractive_index_ratio = air_refractive_index / refractive_index;
                let mut cosine =
                    -ray.direction().dot(&hitpoint.normal) / util::length(ray.direction());

                if ray.direction().dot(&hitpoint.normal) > 0. {
                    outward_normal = -hitpoint.normal;
                    refractive_index_ratio = refractive_index / air_refractive_index;
                    cosine = refractive_index * ray.direction().dot(&hitpoint.normal)
                        / util::length(ray.direction());
                }

                let mut final_vector =
                    util::refraction(ray.direction(), &outward_normal, refractive_index_ratio)
                        .unwrap_or(reflected_vector);

                let reflection_prob = util::schlick(cosine, refractive_index);
                let mut rng = thread_rng();

                if (rng.gen_range(0.0..1.0) as f64) < reflection_prob {
                    final_vector = reflected_vector;
                }

                Some((Ray::new(hitpoint.p, final_vector), attenuation))
            }
        }
    }
}
