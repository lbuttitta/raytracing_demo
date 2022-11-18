use crate::Color;
use crate::scene::Camera;
use crate::scene::Light;
use crate::shape::Shape;

/// The primary container object which is rendered.
pub struct Scene {

    /// The color of the ambient light in this scene.
    ///
    /// The magnitude of the color yields the intensity of the light.
    pub background: Color,

    /// The camera in this scene.
    pub camera: Camera,

    /// The light sources in this scene.
    pub lights: Vec<Light>,

    /// The shapes in this scene.
    pub shapes: Vec<Box<dyn Shape>>,

}
