extern crate image;
extern crate nalgebra;
extern crate pbr;
extern crate rand;
extern crate rayon;
extern crate regex;

pub mod ray;
pub mod sphere;
pub mod hitable;
pub mod camera;
pub mod config;
pub mod material;
pub mod vec_util;

pub use config::{Configuration, Resolution};
use pbr::{MultiBar, ProgressBar};
use nalgebra::{Point3, Vector3};
use rand::{thread_rng, Rng};
use sphere::Sphere;
use camera::Camera;
use std::fs::File;
use image::{GenericImage, Rgba};
use std::f64;
use material::Material;
use rayon::prelude::*;

/// Entry point for the application. Generates a hardcoded world, simulates the ray tracing and
/// finally saves the rendered frame to disk, as specified by the `Configuration`.
pub fn run(cfg: &Configuration) {
    // Fail early in case of I/O errors.
    let ref mut fout = File::create(&cfg.output_filename).unwrap();

    // let metal = Material::Metal(Vector3::new(0.6, 0.5, 0.2), 1.0);
    // let matte = Material::Lambertian(Vector3::new(0.5, 0.5, 0.5));

    // Snowman
    // let world = vec![
    //     Sphere::new(Point3::new(0.0, -0.2, -1.0), 0.3, metal.clone()), // lower body
    //     Sphere::new(Point3::new(0.0, 0.24, -1.0), 0.2, metal.clone()), // upper body
    //     Sphere::new(Point3::new(0.0, 0.52, -0.9), 0.12, metal.clone()), // head
    //     Sphere::new(Point3::new(-0.04, 0.53, -0.80), 0.03, metal.clone()), // left eye
    //     Sphere::new(Point3::new(0.04, 0.53, -0.80), 0.03, metal.clone()), // right eye
    //     Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, matte.clone()), // ground
    // ];

    let world = vec![
        Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Material::Lambertian(Vector3::new(0.1, 0.2, 0.5)),
        ),
        Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Material::Lambertian(Vector3::new(0.8, 0.8, 0.0)),
        ),
        Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Material::Metal(Vector3::new(0.8, 0.6, 0.2), 0.0),
        ),
        Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Material::Dielectric(1.5)),
        Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            -0.45,
            Material::Dielectric(1.5),
        ),
    ];

    // let r = f64::consts::FRAC_PI_4.cos();
    // let world = vec![
    //     Sphere::new(
    //         Point3::new(-r, 0.0, -1.0),
    //         r,
    //         Material::Lambertian(Vector3::new(0.0, 0.0, 1.0)),
    //     ),
    //     Sphere::new(
    //         Point3::new(r, 0.0, -1.0),
    //         r,
    //         Material::Lambertian(Vector3::new(1.0, 0.0, 0.0)),
    //     ),
    // ];

    let aspect_ratio = cfg.resolution.width as f64 / cfg.resolution.height as f64;
    let camera = Camera::new(90.0, aspect_ratio);

    // let mut mb = MultiBar::new();
    // let count = (res.width * res.height) as u64;
    // let mut progress_bar = ProgressBar::new((res.width * res.height) as u64);
    // progress_bar.set_max_refresh_rate(Some(Duration::from_millis(50)));

    let mut img = image::DynamicImage::new_rgb8(cfg.resolution.width, cfg.resolution.height);

    let pixels: Vec<_> = img.pixels().into_iter().collect();
    let transformed_pixels: Vec<_> = pixels
        .into_par_iter()
        .map(|pixel| {
            let mut color: Vector3<f64> = (0..cfg.n_samples)
                .into_iter()
                .map(|_| {
                    let mut rng = thread_rng();
                    let u = (pixel.0 as f64 + rng.gen::<f64>()) / img.width() as f64;
                    let v = (pixel.1 as f64 + rng.gen::<f64>()) / img.height() as f64;

                    camera.get_ray(u, v).color(&world, 0)
                })
                .sum();

            color /= cfg.n_samples as f64;
            color.x = color.x.sqrt();
            color.y = color.y.sqrt();
            color.z = color.z.sqrt();
            color *= 255.99;

            (
                pixel.0,
                img.height() - pixel.1 - 1, // vertical axis is reversed
                Rgba([color.x as u8, color.y as u8, color.z as u8, 255]),
            )
        })
        .collect();

    transformed_pixels.into_iter().for_each(|x| {
        img.put_pixel(x.0, x.1, x.2);
    });

    img.save(fout, image::PNG).unwrap();
    // progress_bar.finish_print("Done!");
}
