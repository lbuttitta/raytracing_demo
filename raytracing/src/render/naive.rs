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
    ) -> Option<(&dyn Shape, Vector3<f64>)> {
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
            .map(|(s, p)| (s.as_ref(), p))
    }
}

impl<'scene> Renderer for NaiveRenderer<'scene> {
    type CastError = !;

    fn cast_ray(
        &self,
        theta: f64,
        phi: f64
    ) -> Result<Color, Self::CastError> {
        // abbreviations
        let camera = &self.scene.camera;
        // rotate the default camera angle by theta leftwards, then phi upwards
        let forward = {
            let unit_up = Unit::new_normalize(camera.up);
            let unit_left = Unit::new_normalize(camera.left());
            Rotation3::from_axis_angle(&unit_up, theta)
                * Rotation3::from_axis_angle(&unit_left, phi)
                * camera.forward
        };
        // if the camera's ray intersects a shape in the scene
        if let Some((s, p)) = self.intersect_ray(camera.pos, forward) {
            // the total intensity of all light sources visible from p
            let mut total_intensity = 0.0;
            let n = s.normal_at(p);
            let n_norm = n.norm();
            for light in self.scene.lights.iter() {
                let d = light.pos - p;
                let d_norm = d.norm();
                // if a ray from p + delta intersects a shape in front of light
                if let Some((_, q)) = self.intersect_ray(p + d * 1e-12, d) {
                    if (q - p).norm() < d_norm {
                        continue;
                    }
                }
                /* if the ray doesn't terminate before the light source,
                 * add to total_intensity the base intensity of the light
                 * source times the cosine of the angle between n and d */
                let mut adj_intensity = light.intensity;
                adj_intensity *= n.dot(&d).abs() / (d_norm * n_norm);
                total_intensity += adj_intensity;
            }
            Ok(s.color_at(p) * total_intensity)
        } else {
            // if no shape is intersected, return the scene's background color
            Ok(self.scene.bg)
        }
    }
}
