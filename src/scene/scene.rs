use crate::Color;
use crate::scene::Camera;
use crate::shape::Shape;

pub struct Scene {
    pub camera: Camera,
    pub bg: Color,
    pub shapes: Vec<Box<dyn Shape>>
}
