use ::anyhow::Result;
use ::nalgebra::Vector3;
use ::pixels::{
    Pixels,
    SurfaceTexture,
    wgpu::Color
};
use ::rayon::iter::{
    IndexedParallelIterator,
    ParallelIterator
};
use ::rayon::slice::ParallelSliceMut;
use ::std::f64::consts::PI;
use ::winit::{
    dpi::LogicalSize,
    event::{
        Event,
        WindowEvent::CloseRequested
    },
    event_loop::{
        ControlFlow,
        EventLoop
    },
    window::WindowBuilder
};

mod camera;
mod scene;
mod triangle;
use camera::Camera;
use scene::Scene;
use triangle::Triangle;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const FOV_HORIZ: f64 = 0.5 * PI;
const FOV_VERT: f64 = 0.375 * PI;

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let size = LogicalSize::new(WIDTH, HEIGHT);
    let window =
        WindowBuilder::new()
        .with_inner_size(size)
        .with_min_inner_size(size)
        .with_resizable(false)
        .build(&event_loop)?;
    let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
    let mut pixels = Pixels::new(size.width, size.height, surface_texture)?;

    let scene = Scene {
        bg: Color::BLACK,
        triangles: vec![
            Triangle {
                a: Vector3::new(1.0, 1.0, 8.0),
                b: Vector3::new(-1.0, 1.0, 8.0),
                c: Vector3::new(0.0, 0.0, 8.0),
                color: Color::BLUE
            },
            Triangle {
                a: Vector3::new(-1.0, 1.0, 8.0),
                b: Vector3::new(-1.0, -1.0, 8.0),
                c: Vector3::new(0.0, 0.0, 8.0),
                color: Color::GREEN
            },
            Triangle {
                a: Vector3::new(-1.0, -1.0, 8.0),
                b: Vector3::new(1.0, -1.0, 8.0),
                c: Vector3::new(0.0, 0.0, 8.0),
                color: Color::RED
            },
            Triangle {
                a: Vector3::new(1.0, -1.0, 8.0),
                b: Vector3::new(1.0, 1.0, 8.0),
                c: Vector3::new(0.0, 0.0, 8.0),
                color: Color::WHITE
            }
        ]
    };
    let camera = Camera {
        pos: Vector3::zeros(),
        dir: *Vector3::z_axis(),
        up:  *Vector3::y_axis(),
    };
    let (width, height) = (size.width as i32, size.height as i32);
    let d_theta = FOV_HORIZ / (width as f64);
    let d_phi = -FOV_VERT / (height as f64);
    pixels
        .get_frame()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, pixel)| {
            let i = i as i32;
            let (x, y) = (i % width, i / width);
            let theta = (x - width / 2) as f64 * d_theta;
            let phi = (y - height / 2) as f64 * d_phi;
            let color = &scene.raster(&camera, theta, phi);
            pixel[0] = (color.r * 256.0) as u8;
            pixel[1] = (color.g * 256.0) as u8;
            pixel[2] = (color.b * 256.0) as u8;
            pixel[3] = 255u8;
        });
    pixels.render()?;

    event_loop.run(move |event, _, ctrl_flow| {
        if let Event::WindowEvent { event: CloseRequested, .. } = event {
            *ctrl_flow = ControlFlow::Exit
        }
    });
}
