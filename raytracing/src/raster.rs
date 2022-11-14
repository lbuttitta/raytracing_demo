use ::rayon::iter::IndexedParallelIterator;
use ::rayon::iter::ParallelIterator;
use crate::Color;
use crate::render::Renderer;

pub fn rasterize_into<'a, R, I, C>(
    renderer: &R,
    iter: I,
    fov_horiz: f64,
    fov_vert: f64,
    width: u32,
    height: u32
) -> Result<(), R::CastError>
    where R: Renderer + Sync,
          I: IndexedParallelIterator<Item = &'a mut C>,
          C: From<Color> + 'a
{
    iter.enumerate()
        .try_for_each(|(i, pixel)| {
        let i = i as u32;
        let (width_f64, height_f64) = (width as f64, height as f64);
        // the coordinates associated with this position in the buffer
        let (x, y) = ((i % width) as f64, (i / height) as f64);
        // the angles associated with those coordinates
        let theta = (x - width_f64 / 2.0) * fov_horiz / width_f64;
        let phi = (y - height_f64 / 2.0) * fov_vert / height_f64;
        // copy the calculated color into the buffer
        *pixel = C::from(renderer.cast_ray(theta, phi)?);
        Ok(())
    })
}
