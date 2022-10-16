use ::nalgebra::{
    Rotation3,
    Unit
};
use ::pixels::wgpu::Color;
use crate::{
    Camera,
    Triangle
};

pub struct Scene {
    pub bg: Color,
    pub triangles: Vec<Triangle>
}

impl Scene {
    pub fn raster(&self, camera: &Camera, theta: f64, phi: f64) -> Color {
        // rotate the default camera angle by theta leftwards and phi upwards
        let dir = {
            let unit_up = Unit::new_normalize(camera.up);
            let unit_left = Unit::new_normalize(camera.left());
            let rot
                = Rotation3::from_axis_angle(&unit_up, theta)
                * Rotation3::from_axis_angle(&unit_left, phi);
            rot * camera.dir
        };
        if let Some((t, _))
            = self.triangles.iter()
            // zip triangles with their intersection points
            .map(|&t| (t, t.intersect_ray(camera.pos, dir)))
            // remove the ones with no intersection point
            .filter_map(|(t, p)| p.map(|_| (t, p)))
            // minimize how far the intersection point is from the camera
            .min_by(|(_, p1), (_, p2)| {
                let d1 = (p1.unwrap() - camera.pos).norm();
                let d2 = (p2.unwrap() - camera.pos).norm();
                // I hope this never blows up in my face
                d1.partial_cmp(&d2).unwrap()
            })
        {
            t.color
        } else {
            self.bg
        }
    }
}
