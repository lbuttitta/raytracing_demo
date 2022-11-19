use ::nalgebra::Vector3;
use crate::Color;

/// A uniform, chromatic point light source.
pub struct Light {

    /// The position of this light source.
    pub pos: Vector3<f64>,

    /// The color of light shed by this light source which contributes to
    /// diffuse reflection.
    pub diffuse_color: Color,

    /// The color of light shed by this light source which contributes to
    /// specular reflection.
    pub specular_color: Color

}
