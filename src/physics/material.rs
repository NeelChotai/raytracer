use glam::Vec3;
use rand::{Rng, RngCore};

pub trait Material: Send + Sync {
    fn colour(&self) -> Vec3;

    fn scatter(&self, normal: Vec3, rng: &mut dyn RngCore) -> Vec3;
}

#[derive(Debug, Clone)]
pub struct Diffuse {
    pub colour: Vec3,
}

impl Material for Diffuse {
    fn colour(&self) -> Vec3 {
        self.colour
    }

    fn scatter(&self, normal: Vec3, rng: &mut dyn RngCore) -> Vec3 {
        let in_sphere = loop {
            let v = Vec3::new(
                rng.gen::<f32>() * 2.0 - 1.0,
                rng.gen::<f32>() * 2.0 - 1.0,
                rng.gen::<f32>() * 2.0 - 1.0,
            );
            if v.length_squared() < 1.0 {
                break v.normalize();
            }
        };

        if in_sphere.dot(normal) > 0.0 {
            in_sphere
        } else {
            -in_sphere
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }

    #[test]
    fn diffuse_colour() {
        let mat = Diffuse {
            colour: Vec3::new(1.0, 0.5, 0.2),
        };
        assert_eq!(mat.colour(), Vec3::new(1.0, 0.5, 0.2));
    }

    #[test]
    fn reflect_perpendicular() {
        let v = Vec3::new(1.0, -1.0, 0.0).normalize();
        let n = Vec3::Y;
        let r = reflect(v, n);
        assert!((r - Vec3::new(1.0, 1.0, 0.0).normalize()).length() < 0.001);
    }
}
