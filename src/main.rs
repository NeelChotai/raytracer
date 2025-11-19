use glam::Vec3;
use image::{Rgb, RgbImage};
use log::info;
use raytracer::physics::Diffuse;
use raytracer::scene::{Camera, PointLight, Sphere};
use raytracer::Scene;
use std::time::Instant;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let (width, height) = (800, 600);
    let samples_per_pixel = 8;

    let scene = Scene {
        camera: Camera::new(
            Vec3::new(0.0, 1.5, 3.0),
            Vec3::new(0.0, 0.0, -3.0),
            50.0,
            width as f32 / height as f32,
        ),
        spheres: vec![
            Sphere::new(
                Vec3::new(0.0, -1000.5, -5.0),
                1000.0,
                Diffuse {
                    colour: Vec3::new(0.8, 0.8, 0.8),
                },
            ),
            Sphere::new(
                Vec3::new(-2.0, 0.0, -4.0),
                0.5,
                Diffuse {
                    colour: Vec3::new(0.9, 0.2, 0.2),
                },
            ),
            Sphere::new(
                Vec3::new(0.0, 0.0, -4.0),
                0.5,
                Diffuse {
                    colour: Vec3::new(0.2, 0.9, 0.2),
                },
            ),
            Sphere::new(
                Vec3::new(2.0, 0.0, -4.0),
                0.5,
                Diffuse {
                    colour: Vec3::new(0.2, 0.2, 0.9),
                },
            ),
            Sphere::new(
                Vec3::new(-1.0, 0.0, -3.0),
                0.5,
                Diffuse {
                    colour: Vec3::new(0.9, 0.9, 0.5),
                },
            ),
            Sphere::new(
                Vec3::new(1.0, 0.0, -3.0),
                0.5,
                Diffuse {
                    colour: Vec3::new(0.5, 0.7, 0.9),
                },
            ),
        ],
        light: PointLight {
            position: Vec3::new(0.0, 4.0, -3.0),
            colour: Vec3::ONE,
            intensity: 2.0,
        },
        background: Vec3::new(0.15, 0.20, 0.30),
    };

    info!(
        "rendering {}x{} ({} samples/pixel)...",
        width, height, samples_per_pixel
    );
    let start = Instant::now();

    let pixels = scene.render(width, height, samples_per_pixel);

    info!("rendered in {:.2}s", start.elapsed().as_secs_f32());

    save("output/render.png", &pixels, width, height);
    info!("saved to output/render.png");
}

fn save(path: &str, pixels: &[Vec3], width: u32, height: u32) {
    let mut img = RgbImage::new(width, height);

    pixels.iter().enumerate().for_each(|(i, pixel)| {
        let x = (i as u32) % width;
        let y = height - 1 - (i as u32) / width; // flip Y-axis

        let rgb = [
            (pixel.x.clamp(0.0, 1.0) * 255.0) as u8,
            (pixel.y.clamp(0.0, 1.0) * 255.0) as u8,
            (pixel.z.clamp(0.0, 1.0) * 255.0) as u8,
        ];

        img.put_pixel(x, y, Rgb(rgb));
    });

    std::fs::create_dir_all("output").expect("failed to create output directory");
    img.save(path).expect("failed to save PNG");
}
