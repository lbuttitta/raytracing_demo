use crate::Color;
use crate::scene::Camera;
use crate::scene::Light;
use crate::shape::Shape;

pub struct Scene {
    pub background: Color,
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub shapes: Vec<Box<dyn Shape>>,
}
