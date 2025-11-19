use crate::physics::Material;
use crate::physics::Ray;
use glam::Vec3;
use std::sync::Arc;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub material: Arc<dyn Material>,
}

pub struct Sphere {
    pub centre: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f32, material: impl Material + 'static) -> Self {
        Self {
            centre,
            radius,
            material: Arc::new(material),
        }
    }

    /// ray/sphere intersection using quadratic formula
    /// None if no intersection
    pub fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let t = [(-half_b - sqrt_d) / a, (-half_b + sqrt_d) / a]
            .into_iter()
            .find(|&t| t > 0.001)?;

        let point = ray.at(t);
        let normal = (point - self.centre).normalize();

        Some(HitRecord {
            point,
            normal,
            t,
            material: Arc::clone(&self.material),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::Diffuse;

    #[test]
    fn sphere_intersection_direct_hit() {
        let sphere = Sphere::new(
            Vec3::new(0.0, 0.0, -5.0),
            1.0,
            Diffuse { colour: Vec3::ONE },
        );
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.intersect(&ray);
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!((hit.t - 4.0).abs() < 0.001);
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn sphere_intersection_miss() {
        let sphere = Sphere::new(
            Vec3::new(10.0, 0.0, -5.0),
            1.0,
            Diffuse { colour: Vec3::ONE },
        );
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0));

        assert!(sphere.intersect(&ray).is_none());
    }

    #[test]
    fn sphere_intersection_behind_ray() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 5.0), 1.0, Diffuse { colour: Vec3::ONE });
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0));

        assert!(sphere.intersect(&ray).is_none());
    }
}
