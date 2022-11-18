use ::nalgebra::Vector3;

/// A directional camera.
pub struct Camera {

    /// The position of this camera.
    pub pos: Vector3<f64>,

    /// The direction of "forward" for this camera.
    ///
    /// Rotation around this vector causes the camera to roll.
    pub forward: Vector3<f64>,

    /// The direction of "upward" for this camera.
    ///
    /// Rotation around this vector causes the camera to yaw.
    pub up: Vector3<f64>
}

impl Camera {

    /// Returns the direction of "leftward" for this camera.
    pub fn left(&self) -> Vector3<f64> {
        self.forward.cross(&self.up)
    }

}
