use ::nalgebra::Vector3;
use ::nalgebra::Matrix3;
use crate::Color;
use crate::shape::Shape;
use crate::shape::Triangle;

pub struct CachingTriangle {
    // the triangle is owned by the container and is immutable
    inner: Triangle,
    // with caching, the scene uses ~4x space, but vastly less time
    normal: Vector3<f64>,
    bary: Matrix3<f64>
}

impl CachingTriangle {
    pub fn new(inner: Triangle) -> Option<CachingTriangle> {
        // the normal vector of the triangle
        let normal = inner.normal();
        // a transformation matrix from barycentric to Cartesian coordinates
        let mut bary = Matrix3::from_columns(&[
            inner.b - inner.a,
            inner.c - inner.a,
            normal
        ]);
        // invert it to obtain a transformation from Cartesian to barycentric
        if bary.try_inverse_mut() {
            Some(CachingTriangle { inner, normal, bary })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> Triangle {
        self.inner
    }
}

impl Shape for CachingTriangle {
    fn color_at(&self, _p: Vector3<f64>) -> Color {
        self.inner.color
    }

    fn intersect_ray(
        &self,
        l0: Vector3<f64>,
        l: Vector3<f64>,
    ) -> Option<Vector3<f64>>
    {
        // if the ray is parallel with the triangle
        if self.normal.dot(&l) == 0.0 {
            return None
        }
        // the coefficient of l in p
        let k = self.normal.dot(&(self.inner.a - l0)) / self.normal.dot(&l);
        // if the ray points away from the triangle
        if k < 0.0 {
            return None
        }
        // the intersection between the line and the coplanar plane
        let p = l0 + k * l;
        // the barycentric coordinates of the intersection
        let q = self.bary * (p - self.inner.a);
        let (v, w) = (q[0], q[1]);
        // if p is contained in the triangle
        if v >= 0.0 && w >= 0.0 && v + w <= 1.0 {
            Some(p)
        } else {
            None
        }
    }
}
