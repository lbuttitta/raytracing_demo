use ::nalgebra::Vector3;
use crate::Color;

/// Implemented by objects which can appear in a `Scene`.
pub trait Shape: Send + Sync {

    /// Returns the color of the implementing shape at `p` in ambient white
    /// light.
    ///
    /// Its implementations may require `p` to be a point on the implementing
    /// shape.
    fn ambient_color_at(&self, p: Vector3<f64>) -> Color;

    /// Returns the color of the diffuse reflection of white light off of the
    /// implementing shape at `p`.
    ///
    /// Its implementations may require `p` to be a point on the implementing
    /// shape.
    fn diffuse_color_at(&self, p: Vector3<f64>) -> Color;

    /// Returns the color of the specular reflection of white light off of the
    /// implementing shape at `p`.
    ///
    /// Its implementations may require `p` to be a point on the implementing
    /// shape.
    fn specular_color_at(&self, p: Vector3<f64>) -> Color;

    /// Returns the shininess of the implementing shape at `p`.
    ///
    /// Its implementations may require `p` to be a point on the implementing
    /// shape.
    fn shininess_at(&self, p: Vector3<f64>) -> f64;

    /// Returns the point at which a ray originating from `l0` in the direction
    /// of `l` intersects the implementing shape.
    ///
    /// Its implementations may require the norm of `l` to be strictly positive
    /// (i.e., nonzero).
    fn intersect_ray(
        &self,
        l0: Vector3<f64>,
        l: Vector3<f64>
    ) -> Option<Vector3<f64>>;

    /// Returns a vector normal to the implementing shape at `p`.
    ///
    /// Its implementations may require `p` to be a point on the implementing
    /// shape.
    fn normal_at(&self, p: Vector3<f64>) -> Vector3<f64>;

}
