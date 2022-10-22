use ::nalgebra::Rotation3;
use ::nalgebra::Unit;
use ::rayon::iter::IntoParallelRefIterator;
use ::rayon::iter::ParallelIterator;
use crate::Color;
use crate::render::Renderer;
use crate::scene::Scene;

pub struct NaiveRenderer<'scene> {
    scene: &'scene Scene
}

impl<'scene> NaiveRenderer<'scene> {
    pub fn new(scene: &'scene Scene) -> Self {
        NaiveRenderer { scene }
    }
}

impl<'scene> Renderer for NaiveRenderer<'scene> {
    type CastError = !;

    fn cast_ray(
        &self,
        theta: f64,
        phi: f64
    ) -> Result<Color, Self::CastError> {
        let scene = self.scene;
        let camera = &scene.camera;
        // rotate the default camera angle by theta leftwards, then phi upwards
        let forward = {
            let unit_up = Unit::new_normalize(camera.up);
            let unit_left = Unit::new_normalize(camera.left());
            let rot
                = Rotation3::from_axis_angle(&unit_up, theta)
                * Rotation3::from_axis_angle(&unit_left, phi);
            rot * camera.forward
        };
        let result = scene.shapes.par_iter()
            // zip shapes with their intersection points
            .map(|s| (s, s.intersect_ray(camera.pos, forward)))
            // remove the ones with no intersection point
            .filter_map(|(s, p)| p.map(|_| (s, p.unwrap())))
            // select the shape closest to the camera
            .min_by(|(_, p1), (_, p2)| {
                let d1 = (p1 - camera.pos).norm();
                let d2 = (p2 - camera.pos).norm();
                d1.partial_cmp(&d2).unwrap()
            })
            // either the shape's color (if Some) or the background's (if None)
            .map_or(scene.bg, |(s, p)| s.color_at(p));
        Ok(result)
    }
}
