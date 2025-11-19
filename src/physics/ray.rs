use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_at_parameter_zero() {
        let ray = Ray::new(Vec3::ZERO, Vec3::X);
        assert_eq!(ray.at(0.0), Vec3::ZERO);
    }

    #[test]
    fn ray_at_parameter_positive() {
        let ray = Ray::new(Vec3::ZERO, Vec3::X);
        assert_eq!(ray.at(5.0), Vec3::new(5.0, 0.0, 0.0));
    }

    #[test]
    fn ray_at_parameter_negative() {
        let ray = Ray::new(Vec3::new(10.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0));
        assert_eq!(ray.at(5.0), Vec3::new(5.0, 0.0, 0.0));
    }
}
