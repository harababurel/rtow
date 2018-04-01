#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate nalgebra;
extern crate rand;

mod ray;
mod sphere;
mod hitable;
mod camera;

use nalgebra::{Point3, Vector3};
use rand::{thread_rng, Rng};
use sphere::Sphere;
use camera::Camera;

pub fn run() {
    let nx = 800;
    let ny = 400;
    let n_samples = 100;

    let world = vec![
        Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point3::new(-0.75, -0.4, -1.0), 0.2),
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0),
    ];

    let camera = Camera {
        origin: Point3::new(0.0, 0.0, 0.0),
        lower_left_corner: Vector3::new(-2.0, -1.0, -1.0),
        horizontal: Vector3::new(4.0, 0.0, 0.0),
        vertical: Vector3::new(0.0, 2.0, 0.0),
    };

    println!("P3\n{} {}\n255\n", nx, ny);

    let mut rng = thread_rng();
    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut pixel: Vector3<f64> = (0..n_samples)
                .into_iter()
                .map(|_| {
                    let u = (x as f64 + rng.gen::<f64>()) / nx as f64;
                    let v = (y as f64 + rng.gen::<f64>()) / ny as f64;

                    camera.get_ray(u, v).color(&world)
                })
                .sum();

            pixel /= n_samples as f64;
            pixel.x = pixel.x.sqrt();
            pixel.y = pixel.y.sqrt();
            pixel.z = pixel.z.sqrt();
            pixel *= 255.99;

            println!("{} {} {}", pixel.x as u8, pixel.y as u8, pixel.z as u8);
        }
    }
}
