use ::nalgebra::Vector3;
use crate::Color;

pub struct Light {
    pub pos: Vector3<f64>,
    pub color: Color,
    pub intensity: f64
}
