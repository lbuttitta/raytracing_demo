use ::nalgebra::Rotation3;
use ::nalgebra::Unit;
use ::nalgebra::Vector3;
use ::rayon::iter::IntoParallelRefIterator;
use ::rayon::iter::ParallelIterator;
use crate::Color;
use crate::render::Renderer;
use crate::scene::Scene;
use crate::shape::Shape;

/// My first attempt at writing a rendering algorithm.
pub struct NaiveRenderer<'scene, 'shape> {

    /// The scene referenced by this renderer.
    scene: &'scene Scene<'shape>

}

impl<'scene, 'shape> NaiveRenderer<'scene, 'shape> {

    /// Creates a renderer which references `scene`.
    pub fn new(scene: &'scene Scene<'shape>) -> Self {
        NaiveRenderer { scene }
    }

    /// Returns the first shape intersected by ray originating from `l0` in the
    /// direction of `l`, as well as the point at which the intersection occurs.
    ///
    /// The norm of `l` must be strictly positive (i.e., nonzero).
    fn intersect_ray(
        &self,
        l0: Vector3<f64>,
        l: Vector3<f64>
    ) -> Option<(&dyn Shape, Vector3<f64>)> {
        self.scene.shapes.par_iter()
            // zip shapes with their intersection points
            .map(|s| (s, s.intersect_ray(l0, l)))
            // remove the shapes with no intersection point
            .filter_map(|(s, p)| p.map(|_| (s, p.unwrap())))
            // select the shape closest to the camera
            .min_by(|(_, p1), (_, p2)| {
                let d1 = (p1 - l0).norm();
                let d2 = (p2 - l0).norm();
                d1.partial_cmp(&d2).unwrap()
            })
            .map(|(s, p)| (s.as_ref(), p))
    }

    /// Returns the sum of the colors of all light sources in the scene
    /// referenced by this renderer which are visible from a shape with normal
    /// vector `n` at `p`.
    fn light_at(&self, p: Vector3<f64>, n: Vector3<f64>) -> Color {
        const DELTA: f64 = 1e-12;

        let n_norm = n.norm();
        self.scene.lights.par_iter()
            .map(|light| {
                let d = light.pos - p;
                let d_norm = d.norm();
                // if a ray from p towards the camera intersects a shape...
                if let Some((_, q)) = self.intersect_ray(p + d * DELTA, d) {
                    // ...and the intersected shape is in front of the camera
                    if (q - p).norm() < d_norm {
                        return Color::BLACK;
                    }
                }
                // otherwise, e.g. if that ray extends at least to the camera
                light.color * n.dot(&d).abs() / (d_norm * n_norm)
            })
            .sum::<Color>()
            + self.scene.background
    }

}

impl Renderer for NaiveRenderer<'_, '_> {

    type CastError = !;

    /// Returns the color visible in the scene referenced by this renderer,
    /// from the camera in that scene, in the direction (relative to the
    /// camera) given by `theta` and `phi`.
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
            Ok(s.color_at(p) * self.light_at(p, s.normal_at(p)))
        } else {
            // if no shape is intersected, return the scene's background color
            Ok(self.scene.background)
        }
    }
}
