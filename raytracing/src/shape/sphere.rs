use ::nalgebra::Vector3;
use crate::Color;
use crate::shape::Shape;

/// A sphere.
#[derive(PartialEq)]
pub struct Sphere {

    /// The origin of this sphere.
    pub o: Vector3<f64>,

    /// The radius of this sphere.
    pub r: f64,

    /// The color of this sphere.
    pub color: Color

}

impl Shape for Sphere {

    /// Returns the color of this sphere at `p`.
    ///
    /// `p` must be a point on this sphere.
    fn color_at(&self, _p: Vector3<f64>) -> Color {
        self.color
    }

    /// Returns the point at which a ray originating from `l0` in the direction
    /// of `l` intersects this sphere, if such a point exists.
    ///
    /// The norm of `l` must be strictly positive (i.e., nonzero).
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
        // the square of the distance from the projection to the intersection
        let k2 = self.r * self.r - (self.o - p).norm_squared();
        if k2 >= 0.0 {
            Some(p - k2.sqrt() * l)
        } else {
            None
        }
    }

    /// Returns a vector normal to this sphere at `p`.
    ///
    /// `p` must be a point on this sphere.
    fn normal_at(&self, p: Vector3<f64>) -> Vector3<f64> {
        p - self.o
    }

}
