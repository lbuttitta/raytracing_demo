use ::nalgebra::Vector3;
use crate::Color;

pub trait Shape: Send + Sync {
    fn color_at(
        &self,
        p: Vector3<f64>
    ) -> Color;

    fn intersect_ray(
        &self,
        l0: Vector3<f64>,
        l: Vector3<f64>
    ) -> Option<Vector3<f64>>;
}
