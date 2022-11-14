use ::nalgebra::Rotation3;
use ::nalgebra::Unit;
use ::nalgebra::Vector3;
use ::rayon::iter::IntoParallelRefIterator;
use ::rayon::iter::ParallelIterator;
use crate::Color;
use crate::render::Renderer;
use crate::scene::Scene;
use crate::shape::Shape;

pub struct NaiveRenderer<'scene> {
    scene: &'scene Scene
}

impl<'scene> NaiveRenderer<'scene> {
    pub fn new(scene: &'scene Scene) -> Self {
        NaiveRenderer { scene }
    }

    fn intersect_ray(
        &self,
        l0: Vector3<f64>,
        l: Vector3<f64>
    ) -> Option<(&Box<dyn Shape>, Vector3<f64>)> {
        self.scene.shapes.par_iter()
            // zip shapes with their intersection points
            .map(|s| (s, s.intersect_ray(l0, l)))
            // remove the ones with no intersection point
            .filter_map(|(s, p)| p.map(|_| (s, p.unwrap())))
            // select the shape closest to the camera
            .min_by(|(_, p1), (_, p2)| {
                let d1 = (p1 - l0).norm();
                let d2 = (p2 - l0).norm();
                d1.partial_cmp(&d2).unwrap()
            })
    }
}

impl<'scene> Renderer for NaiveRenderer<'scene> {
    type CastError = !;

    fn cast_ray(
        &self,
        theta: f64,
        phi: f64
    ) -> Result<Color, Self::CastError> {
        // rotate the default camera angle by theta leftwards, then phi upwards
        let forward = {
            let unit_up = Unit::new_normalize(self.scene.camera.up);
            let unit_left = Unit::new_normalize(self.scene.camera.left());
            Rotation3::from_axis_angle(&unit_up, theta)
                * Rotation3::from_axis_angle(&unit_left, phi)
                * self.scene.camera.forward
        };
        /* maybe find the first shape intersected, then return either its
           color (if the shape exists) or the background's (if not) */
        Ok(self.intersect_ray(self.scene.camera.pos, forward)
            .map_or(self.scene.bg, |(s, p)| s.color_at(p)))
    }
}
