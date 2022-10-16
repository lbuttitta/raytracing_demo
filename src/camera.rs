use ::nalgebra::Vector3;

pub struct Camera {
    pub pos: Vector3<f64>,
    pub dir: Vector3<f64>,
    pub up: Vector3<f64>
}

impl Camera {
    pub fn left(&self) -> Vector3<f64> {
        self.dir.cross(&self.up)
    }
}
