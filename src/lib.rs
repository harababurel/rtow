#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate nalgebra;
extern crate rand;

mod ray;
mod sphere;
mod hitable;
mod camera;

use nalgebra::Vector3 as Vector;
use nalgebra::Point3 as Point;
use rand::{thread_rng, Rng};
use sphere::Sphere;
use camera::Camera;

pub fn run() {
    let nx = 800;
    let ny = 400;
    let n_samples = 100;

    let lower_left_corner = Vector::new(-2.0, -1.0, -1.0);
    let horizontal = Vector::new(4.0, 0.0, 0.0);
    let vertical = Vector::new(0.0, 2.0, 0.0);
    let origin = Point::new(0.0, 0.0, 0.0);

    let world = vec![
        Sphere::new(&Point::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(&Point::new(0.0, -100.5, -1.0), 100.0),
    ];

    println!("P3\n{} {}\n255\n", nx, ny);

    let camera = Camera {
        origin,
        lower_left_corner,
        horizontal,
        vertical,
    };

    let mut rng = thread_rng();
    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut pixel = Vector::new(0.0, 0.0, 0.0);
            for _ in 0..n_samples {
                let u = (x as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (y as f32 + rng.gen::<f32>()) / ny as f32;

                let ray = camera.get_ray(u, v);
                pixel += ray.color(&world);
            }

            pixel /= n_samples as f32;
            pixel *= 255.99;

            println!("{} {} {}", pixel.x as u16, pixel.y as u16, pixel.z as u16);
        }
    }
}
