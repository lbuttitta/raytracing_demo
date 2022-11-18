/// Contains `Shape`, a trait implemented by objects which can appear in a
/// `Scene`, as well as various types that implement `Shape`.

mod shape;
mod sphere;
mod triangle;

pub use shape::*;
pub use sphere::*;
pub use triangle::*;
