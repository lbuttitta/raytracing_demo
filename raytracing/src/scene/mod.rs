/// Contains `Scene`, the primary object which is rendered, as well as some of
/// its components, `Camera` and `Light` (but not `Shape`).

mod camera;
mod light;
mod scene;

pub use camera::*;
pub use light::*;
pub use scene::*;
