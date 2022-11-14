#![feature(never_type)]

pub mod render;
pub mod scene;
pub mod shape;

mod color;
mod raster;

pub use color::*;
pub use raster::*;
