extern crate chan;
extern crate image;
#[macro_use]
extern crate log;
extern crate nalgebra;
extern crate pretty_env_logger;
extern crate rand;
extern crate regex;

pub mod camera;
pub mod config;
pub mod hitable;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod util;

use camera::{Camera, Lens, Orientation};
pub use config::{Config, Resolution};
use image::{GenericImage, GenericImageView, Rgba};
use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};
use material::Material;
use material::Material::{Lambertian, Metal};
use nalgebra::{Point3, Vector3};
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use sphere::Sphere;
use std::f64;
use std::thread;

const EARTH_RADIUS: f64 = 6.371e6;

fn random_scene(object_count: u32) -> Vec<Sphere> {
    let ground = Sphere::new(
        Point3::new(0., -EARTH_RADIUS, 0.),
        EARTH_RADIUS,
        Lambertian {
            attenuation: Vector3::new(0.5, 0.5, 0.5),
        },
    );

    let metal_sphere = Sphere::new(
        Point3::new(0., 1., -4.),
        1.,
        Metal {
            attenuation: Vector3::new(0.7, 0.6, 0.5),
            fuzziness: 0.,
        },
    );
    let glass_sphere = Sphere::new(Point3::new(0., 1., -8.), 1., Material::random_dielectric());
    let matte_sphere = Sphere::new(Point3::new(0., 1., -12.), 1., Material::random_lambertian());

    let mut world = vec![ground.clone(), glass_sphere, matte_sphere, metal_sphere];
    let mut rng = thread_rng();

    for _ in 0..object_count {
        loop {
            let x: f64 = rng.gen_range(-30.0..30.0);
            let z: f64 = rng.gen_range(-60.0..5.0);
            let radius: f64 = rng.gen_range(0.1..0.5);

            // Account for the curvature of the earth.
            let sea_level =
                (ground.radius().powf(2.) - (x - ground.x()).powf(2.) - (z - ground.z()).powf(2.))
                    .sqrt()
                    + ground.y();
            debug!("sea level: {}", sea_level);

            let y = sea_level + radius;
            let sphere = Sphere::new(Point3::new(x, y, z), y, Material::random_material());

            if world.iter().any(|existing| existing.intersects(&sphere)) {
                continue;
            } else {
                world.push(sphere);
                break;
            }
        }
    }

    for sphere in &world {
        info!("Sphere: {}", sphere.center());
    }

    world
}

/// Entry point for the application. Generates a hardcoded world, simulates the ray tracing and
/// finally saves the rendered frame to disk, as specified by the `Config`.
pub fn run(cfg: Config) {
    // Fail early in case of I/O errors.
    // let mut ref fout = File::create(&cfg.output_filename).unwrap();

    let world = random_scene(500);

    let orientation = Orientation {
        look_from: Point3::new(-2., 1.7, 0.),
        look_at: Point3::new(0., 1., -8.),
        upwards: Vector3::new(0., 1., 0.),
    };
    let lens = Lens {
        aperture: 0.1,
        focal_length: util::length(&(orientation.look_from - orientation.look_at)),
        vertical_fov: 50.,
        aspect_ratio: cfg.resolution.width as f64 / cfg.resolution.height as f64,
    };

    let camera = Camera::new(orientation, lens);

    let r = {
        let (s, r): (chan::Sender<_>, chan::Receiver<_>) = chan::r#async();

        let cfg = cfg.clone();
        thread::spawn(move || {
            let mut pixels: Vec<(u32, u32, image::Rgba<u8>)> =
                image::DynamicImage::new_rgb8(cfg.resolution.width, cfg.resolution.height)
                    .pixels()
                    .into_iter()
                    .collect();

            // Some areas of the image take more time to render. This makes the progress bar
            // advance unevenly. Shuffling the pixels leads to a more even distribution and a more
            // accurate ETA.
            pixels.shuffle(&mut thread_rng());
            pixels.into_iter().for_each(|pixel| s.send(pixel));
        });
        r
    };

    let (ret_s, ret_r) = chan::r#async();
    let wg = chan::WaitGroup::new();

    for _ in 0..cfg.threads {
        wg.add(1);

        let wg = wg.clone();
        let r = r.clone();
        let cfg = cfg.clone();
        let world = world.clone();
        let ret_s = ret_s.clone();
        thread::spawn(move || {
            for pixel in r {
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
    let pb = ProgressBar::new((cfg.resolution.width * cfg.resolution.height).into());
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar}] {pos}/{len} [{percent}%] [{per_sec}] [{eta_precise}]"
            )
            .progress_chars("##-")
            .on_finish(ProgressFinish::AndLeave),
    );
    for pixel in ret_r {
        img.put_pixel(pixel.0, pixel.1, pixel.2);
        pb.inc(1);
    }
    wg.wait();
    img.save(cfg.output_filename).unwrap();
}
