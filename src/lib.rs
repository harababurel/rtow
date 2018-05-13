extern crate chan;
extern crate image;
extern crate log;
extern crate nalgebra;
extern crate pbr;
extern crate pretty_env_logger;
extern crate rand;
extern crate regex;

pub mod camera;
pub mod config;
pub mod hitable;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec_util;

use camera::{Camera, Orientation};
pub use config::{Configuration, Resolution};
use image::{GenericImage, Rgba};
use material::Material;
use nalgebra::{Point3, Vector3};
use pbr::ProgressBar;
use rand::{thread_rng, Rng};
use sphere::Sphere;
use std::f64;
use std::fs::File;
use std::thread;

/// Entry point for the application. Generates a hardcoded world, simulates the ray tracing and
/// finally saves the rendered frame to disk, as specified by the `Configuration`.
pub fn run(cfg: Configuration) {
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

    let orientation = Orientation {
        look_from: Point3::new(-2.0, 2.0, 1.0),
        look_at: Point3::new(0.0, 0.0, -1.0),
        upwards: Vector3::new(0.0, 1.0, 0.0),
    };

    let aspect_ratio = cfg.resolution.width as f64 / cfg.resolution.height as f64;
    let camera = Camera::new(orientation, 40.0, aspect_ratio);

    let r = {
        let (s, r): (chan::Sender<_>, chan::Receiver<_>) = chan::async();

        let cfg = cfg.clone();
        thread::spawn(move || {
            image::DynamicImage::new_rgb8(cfg.resolution.width, cfg.resolution.height)
                .pixels()
                .into_iter()
                .for_each(|pixel| {
                    // println!("sending pixel");
                    s.send(pixel)
                });
        });
        r
    };

    let (ret_s, ret_r): (chan::Sender<_>, chan::Receiver<_>) = chan::async();
    let wg = chan::WaitGroup::new();

    for _ in 0..cfg.n_threads {
        wg.add(1);
        let wg = wg.clone();
        let r = r.clone();

        let cfg = cfg.clone();
        let world = world.clone();
        let ret_s = ret_s.clone();
        thread::spawn(move || {
            for pixel in r {
                // println!("Received pixel: {:?}", pixel);

                let mut color: Vector3<f64> = (0..cfg.n_samples)
                    .into_iter()
                    .map(|_| {
                        let mut rng = thread_rng();
                        let u = (pixel.0 as f64 + rng.gen::<f64>()) / cfg.resolution.width as f64;
                        let v = (pixel.1 as f64 + rng.gen::<f64>()) / cfg.resolution.height as f64;

                        camera.get_ray(u, v).color(&world, 0)
                    })
                    .sum();

                color /= cfg.n_samples as f64;
                color.x = color.x.sqrt();
                color.y = color.y.sqrt();
                color.z = color.z.sqrt();
                color *= 255.99;

                let transformed_pixel = (
                    pixel.0,
                    cfg.resolution.height - pixel.1 - 1, // vertical axis is reversed
                    Rgba([color.x as u8, color.y as u8, color.z as u8, 255]),
                );
                ret_s.send(transformed_pixel);
            }
            wg.done();
        });
    }
    drop(ret_s);

    let mut img = image::DynamicImage::new_rgb8(cfg.resolution.width, cfg.resolution.height);
    let mut pb = ProgressBar::new((cfg.resolution.width * cfg.resolution.height).into());
    for pixel in ret_r {
        img.put_pixel(pixel.0, pixel.1, pixel.2);
        pb.inc();
    }

    wg.wait();
    img.save(fout, image::PNG).unwrap();
    pb.finish_print("Done!");
}
