#![feature(never_type)]
mod color;
pub mod raster;
pub mod render;
pub mod scene;
pub mod shape;

use ::anyhow::Result;
use ::nalgebra::Vector3;
use ::pixels::Pixels;
use ::pixels::SurfaceTexture;
use ::rayon::iter::ParallelIterator;
use ::rayon::slice::ParallelSliceMut;
use ::std::f64::consts::PI;
use ::time::Instant;
use ::winit::dpi::PhysicalSize;
use ::winit::event::Event;
use ::winit::event::WindowEvent;
use ::winit::event_loop::ControlFlow;
use ::winit::event_loop::EventLoop;
use ::winit::window::WindowBuilder;
use self::color::Color;
use self::raster::rasterize_into;
use self::render::NaiveRenderer;
use self::scene::Camera;
use self::scene::Scene;
use self::shape::Triangle;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const FOV_HORIZ: f64 = 0.5 * PI;
const FOV_VERT: f64 = 0.375 * PI;

fn main() -> Result<()> {
    // the controller for new windows
    let event_loop = EventLoop::new();

    // create a new window and obtain a drawing surface for it
    let mut size = PhysicalSize::new(WIDTH, HEIGHT);
    let window =
        WindowBuilder::new()
        .with_inner_size(size)
        .with_min_inner_size(size)
        .with_resizable(false)
        .build(&event_loop)?;
    let mut pixels = {
        let surface_texture = SurfaceTexture::new(
            size.width,
            size.height,
            &window
        );
        Pixels::new(size.width, size.height, surface_texture)?
    };

    // create a scene and a renderer for it
    let mut scene = Scene {
        bg: Color { r: 63, g: 63, b: 63 },
        camera: Camera {
            pos: Vector3::new(0.0, 0.0, 8.0),
            forward: Vector3::new(0.0, 0.0, -1.0),
            up:  Vector3::new(0.0, 1.0, 0.0),
        },
        shapes: vec![
            Box::new(Triangle {
                a: Vector3::new(1.0, 1.0, 0.0),
                b: Vector3::new(-1.0, 1.0, 0.0),
                c: Vector3::new(0.0, 0.0, 1.0),
                color: Color::BLACK
            }),
            Box::new(Triangle {
                a: Vector3::new(-1.0, 1.0, 0.0),
                b: Vector3::new(-1.0, -1.0, 0.0),
                c: Vector3::new(0.0, 0.0, 1.0),
                color: Color::RED
            }),
            Box::new(Triangle {
                a: Vector3::new(-1.0, -1.0, 0.0),
                b: Vector3::new(1.0, -1.0, 0.0),
                c: Vector3::new(0.0, 0.0, 1.0),
                color: Color::GREEN
            }),
            Box::new(Triangle {
                a: Vector3::new(-1.0, 1.0, 0.0),
                b: Vector3::new(-1.0, -1.0, 0.0),
                c: Vector3::new(0.0, 0.0, -1.0),
                color: Color::YELLOW
            }),
            Box::new(Triangle {
                a: Vector3::new(1.0, -1.0, 0.0),
                b: Vector3::new(1.0, 1.0, 0.0),
                c: Vector3::new(0.0, 0.0, 1.0),
                color: Color::BLUE
            }),
            Box::new(Triangle {
                a: Vector3::new(1.0, 1.0, 0.0),
                b: Vector3::new(-1.0, 1.0, 0.0),
                c: Vector3::new(0.0, 0.0, -1.0),
                color: Color::MAGENTA
            }),
            Box::new(Triangle {
                a: Vector3::new(1.0, -1.0, 0.0),
                b: Vector3::new(1.0, 1.0, 0.0),
                c: Vector3::new(0.0, 0.0, -1.0),
                color: Color::CYAN
            }),
            Box::new(Triangle {
                a: Vector3::new(-1.0, -1.0, 0.0),
                b: Vector3::new(1.0, -1.0, 0.0),
                c: Vector3::new(0.0, 0.0, -1.0),
                color: Color::WHITE
            })
        ]
    };
    
    let start_time = Instant::now();

    event_loop.run(move |event, _, ctrl_flow| {
        ctrl_flow.set_poll();
        match event {
            Event::MainEventsCleared => {
                let t = (Instant::now() - start_time).as_seconds_f64();
                // mutably borrow the scene and update its camera
                {
                    let scene = &mut scene;
                    scene.camera.pos[0] = t.cos() * 8.0;
                    scene.camera.pos[2] = t.sin() * 8.0;
                    scene.camera.forward[0] = -t.cos();
                    scene.camera.forward[2] = -t.sin();
                }
                // create a new renderer for the updated scene
                let renderer = NaiveRenderer::new(&scene);
                // is there some way of using the ? operator here?
                rasterize_into(
                    &renderer,
                    pixels
                        .get_frame()
                        .par_chunks_exact_mut(4)
                        // convert the [u8] slices into [u8; 4] arrays
                        .map(|pixel| {
                            TryInto::<&mut [u8; 4]>::try_into(pixel).unwrap()
                        }),
                    FOV_HORIZ,
                    FOV_VERT,
                    size.width,
                    size.height
                ).unwrap();
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                pixels.render().unwrap();
            },
            Event::WindowEvent { event: WindowEvent::Resized(size2), .. } => {
                size = size2;
                pixels.resize_buffer(size.width, size.height);
                pixels.resize_surface(size.width, size.height);
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *ctrl_flow = ControlFlow::Exit
            },
            _ => {}
        }
    });
}
