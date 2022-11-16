use ::nalgebra::Vector3;
use crate::Color;
use crate::shape::Shape;

#[derive(PartialEq)]
pub struct Sphere {
    pub o: Vector3<f64>,
    pub r: f64,
    pub color: Color
}

impl Shape for Sphere {
    fn color_at(&self, _p: Vector3<f64>) -> Color {
        self.color
    }

    fn intersect_ray(
        &self,
        l0: Vector3<f64>,
        l: Vector3<f64>,
    ) -> Option<Vector3<f64>>
    {
        // the displacement from the ray's origin to the sphere's center
        let d = self.o - l0;
        /* if the ray points away from the sphere's center and its origin is
         * outside the sphere */
        if d.dot(&l) <= 0.0 && d.norm() >= self.r {
            return None
        }
        // the projection of the sphere's center onto the ray
        let p = l0 + d.dot(&l) / l.norm_squared() * l;
        // the distance from the projection to the intersection
        let k = (self.r * self.r - (self.o - p).norm_squared()).sqrt();
        Some(l0 + ((p - l0).norm() - k) * l)
    }

    fn normal_at(&self, p: Vector3<f64>) -> Vector3<f64> {
        p - self.o
    }
}
