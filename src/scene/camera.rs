use crate::physics::Ray;
use glam::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, look_at: Vec3, vfov: f32, aspect: f32) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect * viewport_height;

        let w = (origin - look_at).normalize();
        let u = Vec3::Y.cross(w).normalize();
        let v = w.cross(u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin,
            lower_left,
            horizontal,
            vertical,
        }
    }

    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            (self.lower_left + u * self.horizontal + v * self.vertical - self.origin).normalize(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_ray_centre() {
        let camera = Camera::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0), 90.0, 1.0);
        let ray = camera.ray(0.5, 0.5);
        assert_eq!(ray.origin, Vec3::ZERO);
        // roughly forwards
        assert!(ray.direction.z < 0.0);
    }
}
