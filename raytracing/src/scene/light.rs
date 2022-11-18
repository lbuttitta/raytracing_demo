use ::nalgebra::Vector3;
use crate::Color;

/// A uniform, chromatic point light source.
pub struct Light {

    /// The position of this light source.
    pub pos: Vector3<f64>,

    /// The color of the light shed by this light source.
    /// 
    /// The magnitude of the color yields the intensity of the light source.
    pub color: Color

}
