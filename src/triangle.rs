use ::nalgebra::{
    Matrix3x2,
    Vector3
};
use ::pixels::wgpu::Color;

#[derive(Copy, Clone, PartialEq)]
pub struct Triangle {
    pub a: Vector3<f64>,
    pub b: Vector3<f64>,
    pub c: Vector3<f64>,
    pub color: Color
}

impl Triangle {
    pub fn normal(&self) -> Vector3<f64> {
        (self.b - self.a).cross(&(self.c - self.a))
    }
    
    pub fn intersect_ray(
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
        // the intersection between the line and the plane
        let p = l0 + k * l;
        // transforms from Cartesian to barycentric coordinates
        let m = Matrix3x2::from_columns(&[
            self.b - self.a,
            self.c - self.a
        ]).pseudo_inverse(0.0);
        if let Ok(m) = m {
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
}
