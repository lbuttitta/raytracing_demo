use ::std::error::Error;
use crate::Color;

pub trait Renderer: Sized {
    type CastError: Error + Send;

    fn cast_ray(
        &self,
        theta: f64,
        phi: f64
    ) -> Result<Color, Self::CastError>;
}
