use ::std::error::Error;
use crate::Color;

/// Implemented by algorithms which can render a `Scene`.
pub trait Renderer: Sized {

    type CastError: Error + Send;

    /// Returns the color visible in the direction given by `theta` and `phi`.
    fn cast_ray(
        &self,
        theta: f64,
        phi: f64
    ) -> Result<Color, Self::CastError>;

}
