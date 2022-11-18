use ::nalgebra::Matrix3;
use ::nalgebra::Vector3;
use crate::Color;
use crate::shape::Shape;

/// A triangle.
#[derive(PartialEq)]
pub struct Triangle {

    /// The first vertex of this triangle.
    pub a: Vector3<f64>,

    /// The second vertex of this triangle.
    pub b: Vector3<f64>,

    /// The third vertex of this triangle.
    pub c: Vector3<f64>,

    /// The color of this triangle.
    pub color: Color

}

impl Triangle {

    /// Returns a vector normal to this triangle.
    fn normal(&self) -> Vector3<f64> {
        (self.b - self.a).cross(&(self.c - self.a))
    }

}

impl Shape for Triangle {

    /// Returns the color of this triangle at `p`.
    ///
    /// `p` must be a point on this triangle.
    fn color_at(&self, _p: Vector3<f64>) -> Color {
        self.color
    }

    /// Returns the point at which a ray originating from `l0` in the direction
    /// of `l` intersects this triangle, if such a point exists.
    ///
    /// The norm of `l` must be strictly positive (i.e., nonzero).
    fn intersect_ray(
        &self,
        l0: Vector3<f64>,
        l: Vector3<f64>,
    ) -> Option<Vector3<f64>>
    {
        let n = self.normal();
        // if the ray is parallel with the triangle
        if n.dot(&l) == 0.0 {
            return None
        }
        // the coefficient of l in p
        let k = n.dot(&(self.a - l0)) / n.dot(&l);
        // if the ray points away from the triangle
        if k < 0.0 {
            return None
        }
        // the intersection between the line and the coplanar plane
        let p = l0 + k * l;
        // a transformation matrix from barycentric to Cartesian coordinates
        let mut m = Matrix3::from_columns(&[
            self.b - self.a,
            self.c - self.a,
            n
        ]);
        // invert it to obtain a transformation from Cartesian to barycentric
        if m.try_inverse_mut() {
            let q = m * (p - self.a);
            let (v, w) = (q[0], q[1]);
            // if p is contained in the triangle
            if v >= 0.0 && w >= 0.0 && v + w <= 1.0 {
                Some(p)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns a vector normal to this triangle at `p`.
    ///
    /// `p` must be a point on this triangle.
    fn normal_at(&self, _p: Vector3<f64>) -> Vector3<f64> {
        self.normal()
    }
}
