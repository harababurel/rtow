#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate image;
extern crate nalgebra;
extern crate pbr;
extern crate rand;
extern crate regex;

mod ray;
mod sphere;
mod hitable;
mod camera;
mod config;

pub use config::{Configuration, Resolution};
use pbr::ProgressBar;
use nalgebra::{Point3, Vector3};
use rand::{thread_rng, Rng};
use sphere::Sphere;
use camera::Camera;
use std::fs::File;
use std::time::Duration;
use image::Pixel;

pub fn run(cfg: &Configuration) {
    // Fail early in case of I/O errors.
    let ref mut fout = File::create(&cfg.output_filename).unwrap();

    // Snowman
    let world = vec![
        Sphere::new(Point3::new(0.0, -0.2, -1.0), 0.3), // lower body
        Sphere::new(Point3::new(0.0, 0.24, -1.0), 0.2), // upper body
        Sphere::new(Point3::new(0.0, 0.52, -0.9), 0.12), // head
        Sphere::new(Point3::new(-0.04, 0.53, -0.80), 0.03), // left eye
        Sphere::new(Point3::new(0.04, 0.53, -0.80), 0.03), // right eye
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0), // ground
    ];

    let camera = Camera {
        origin: Point3::new(0.0, 0.1, 0.0),
        lower_left_corner: Vector3::new(-2.0, -1.0, -1.0),
        horizontal: Vector3::new(4.0, 0.0, 0.0),
        vertical: Vector3::new(0.0, 2.0, 0.0),
    };

    let res = &cfg.resolution;
    let mut progress_bar = ProgressBar::new((res.width * res.height) as u64);
    progress_bar.set_max_refresh_rate(Some(Duration::from_millis(50)));
    let mut rng = thread_rng();
    let mut img_buf = image::ImageBuffer::new(res.width, res.height);

    for y in 0..res.height {
        for x in 0..res.width {
            let mut pixel: Vector3<f64> = (0..cfg.n_samples)
                .into_iter()
                .map(|_| {
                    let u = (x as f64 + rng.gen::<f64>()) / res.width as f64;
                    let v = (y as f64 + rng.gen::<f64>()) / res.height as f64;

                    camera.get_ray(u, v).color(&world)
                })
                .sum();

            pixel /= cfg.n_samples as f64;
            pixel.x = pixel.x.sqrt();
            pixel.y = pixel.y.sqrt();
            pixel.z = pixel.z.sqrt();
            pixel *= 255.99;

            img_buf.put_pixel(
                x,
                res.height - y - 1,
                image::Rgb::from_channels(pixel.x as u8, pixel.y as u8, pixel.z as u8, 255),
            );
            progress_bar.inc();
        }
    }
    image::ImageRgb8(img_buf).save(fout, image::PNG).unwrap();
    progress_bar.finish_print("Done!");
}
