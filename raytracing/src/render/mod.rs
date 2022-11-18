/// Contains `Renderer`, a trait implemented by algorithms which can render a
/// `Scene`, as well as various types that implement `Renderer`.

mod naive;
mod renderer;

pub use naive::*;
pub use renderer::*;
