use crate::physics::Ray;
use crate::scene::{Camera, HitRecord, PointLight, Sphere};
use glam::Vec3;
use rand::Rng;
use rayon::prelude::*;

const RAY_EPSILON: f32 = 0.001;

pub struct Scene {
    pub camera: Camera,
    pub spheres: Vec<Sphere>,
    pub light: PointLight,
    pub background: Vec3,
}

impl Scene {
    pub fn render(&self, width: u32, height: u32, samples_per_pixel: u32) -> Vec<Vec3> {
        (0..height)
            .into_par_iter()
            .flat_map(|y| {
                (0..width)
                    .into_par_iter()
                    .map(move |x| self.render_pixel(x, y, width, height, samples_per_pixel))
            })
            .collect()
    }

    fn render_pixel(&self, x: u32, y: u32, width: u32, height: u32, samples: u32) -> Vec3 {
        let mut rng = rand::thread_rng();

        // anti-aliasing via jittered sampling
        let colour: Vec3 = (0..samples)
            .map(|_| {
                let u = (x as f32 + rng.gen::<f32>()) / (width - 1) as f32;
                let v = (y as f32 + rng.gen::<f32>()) / (height - 1) as f32;
                let ray = self.camera.ray(u, v);
                self.trace_ray(&ray, 0)
            })
            .sum();

        let avg = colour / samples as f32;

        // correct gamma
        Vec3::new(avg.x.sqrt(), avg.y.sqrt(), avg.z.sqrt())
    }

    fn trace_ray(&self, ray: &Ray, depth: u32) -> Vec3 {
        const MAX_DEPTH: u32 = 3;

        if depth >= MAX_DEPTH {
            return self.background;
        }

        let hit = self
            .spheres
            .iter()
            .filter_map(|sphere| sphere.intersect(ray))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        match hit {
            Some(hit) => self.shade(&hit),
            None => self.background,
        }
    }

    fn shade(&self, hit: &HitRecord) -> Vec3 {
        let to_light_vector = self.light.position - hit.point;
        let distance = to_light_vector.length();
        let to_light = to_light_vector.normalize();

        let shadow_ray = Ray::new(hit.point + hit.normal * RAY_EPSILON, to_light);

        let occluded = self
            .spheres
            .iter()
            .filter_map(|s| s.intersect(&shadow_ray))
            .any(|h| h.t < distance);

        if occluded {
            self.background * 0.1
        } else {
            let n_dot_l = hit.normal.dot(to_light).max(0.0);
            hit.material.colour() * self.light.colour * self.light.intensity * n_dot_l
        }
    }
}
