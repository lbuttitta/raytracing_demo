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
            /* zip shapes with their intersection points and filter out the
             * ones with no such point */
            .filter_map(|s| s.intersect_ray(l0, l).map(|p| (s.as_ref(), p)))
            // select the shape closest to the camera
            .min_by(|(_, p1), (_, p2)| {
                let d1 = (p1 - l0).norm();
                let d2 = (p2 - l0).norm();
                d1.partial_cmp(&d2).unwrap()
            })
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
            // a vector normal to the shape at the intersection point
            let n = s.normal_at(p);
            let n_norm = n.norm();
            // the displacement from p to the camera
            let dc = camera.pos - p;
            let dc_norm = dc.norm();
            // the shininess of the shape at the intersection point
            let h = s.shininess_at(p);

            let lights = self.scene.lights.par_iter()
                /* filter out the lights such that a ray from p to it is
                 * intersected by a shape in front of it */
                .filter(|light| {
                    // the displacement from p to the light source
                    let dl = light.pos - p;
                    /* returns true if the ray has no intersection point or if
                     * the point is farther from p than the light source */
                    self.intersect_ray(p + dl * 1.0e-12, dl)
                        .map_or(true, |(_, q)| (p - q).norm() > dl.norm())
                });
            let tot_ambient = self.scene.ambient_color * s.ambient_color_at(p);
            // the incoming light to be reflected
            let (sum_diffuse, sum_specular) = lights.map(|light| {
                // the displacement from p to the light source
                let dl = light.pos - p;
                // the diffuse reflection from the light source
                let diffuse = light.diffuse_color
                    * dl.dot(&n) / (n_norm * dl.norm());
                // the reflection of dl across n
                let r = 2.0 * dl.dot(&n).abs() / (dl.norm() * n_norm) * n - dl;
                // the specular reflection from the light source
                let specular = light.specular_color
                    * (r.dot(&dc).abs() / (r.norm() * dc_norm)).powf(h);
                (diffuse, specular)
            })
            .reduce(
                || (Color::BLACK, Color::BLACK),
                |(sum_diffuse, sum_specular), (diffuse, specular)| {
                    (
                        sum_diffuse + diffuse,
                        if diffuse != Color::BLACK {
                            sum_specular + specular
                        } else {
                            sum_specular
                        }
                    )
                }
            );
            let tot_diffuse = s.diffuse_color_at(p) * sum_diffuse;
            let tot_specular = s.specular_color_at(p) * sum_specular;
            Ok(tot_ambient + tot_diffuse + tot_specular)
        } else {
            // if no shape is intersected, return the scene's background color
            Ok(self.scene.background_color)
        }
    }

}
