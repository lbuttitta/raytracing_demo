use ::anyhow::Result;
use ::nalgebra::Vector3;
use ::pixels::Pixels;
use ::pixels::SurfaceTexture;
use ::rayon::iter::ParallelIterator;
use ::rayon::slice::ParallelSliceMut;
use ::raytracing::Color;
use ::raytracing::rasterize_into;
use ::raytracing::render::NaiveRenderer;
use ::raytracing::scene::Camera;
use ::raytracing::scene::Light;
use ::raytracing::scene::Scene;
use ::raytracing::shape::Sphere;
use ::std::f64::consts::PI;
use ::time::Instant;
use ::winit::dpi::PhysicalSize;
use ::winit::event::Event;
use ::winit::event::WindowEvent;
use ::winit::event_loop::ControlFlow;
use ::winit::event_loop::EventLoop;
use ::winit::window::WindowBuilder;

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
    let mut scene = construct_scene();

    let start_time = Instant::now();

    event_loop.run(move |event, _, ctrl_flow| {
        ctrl_flow.set_poll();
        match event {
            Event::MainEventsCleared => {
                let t = (Instant::now() - start_time).as_seconds_f64();
                // mutably borrow the scene and update its camera and lighting
                update_scene(&mut scene, t);
                // create a new renderer for the updated scene
                let renderer = NaiveRenderer::new(&scene);
                rasterize_into(
                    &renderer,
                    pixels
                        .get_frame_mut()
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

fn construct_scene() -> Scene<'static> {
    Scene {
        background_color: Color::BLACK,
        ambient_color: Color::WHITE * 0.05,
        camera: Camera {
            pos: Vector3::new(-5.0, 2.5, 0.0),
            forward: Vector3::new(3.0f64.sqrt() / 2.0, -0.5, 0.0),
            up: Vector3::new(0.5, 3.0f64.sqrt() / 2.0, 0.0),
        },
        lights: vec![
            Light {
                pos: Vector3::new(0.0, 0.0, 0.0),
                diffuse_color: Color::WHITE,
                specular_color: Color::WHITE * 0.5
            }
        ],
        shapes: vec![]
    }
}

fn update_scene(scene: &mut Scene, t: f64) {
    scene.shapes = vec![
        Box::new(Sphere {
            o: Vector3::new(t.cos(), 0.0, t.sin()),
            r: 0.25,
            ambient_color: Color::RED,
            diffuse_color: Color::RED,
            specular_color: Color::RED,
            shininess: 2.0
        }),
        Box::new(Sphere {
            o: 2.0 * Vector3::new((t / 2.0).cos(), 0.0, (t / 2.0).sin()),
            r: 0.5,
            ambient_color: Color::GREEN,
            diffuse_color: Color::GREEN,
            specular_color: Color::GREEN,
            shininess: 2.0
        }),
        Box::new(Sphere {
            o: 3.0 * Vector3::new((t / 3.0).cos(), 0.0, (t / 3.0).sin()),
            r: 1.0,
            ambient_color: Color::BLUE,
            diffuse_color: Color::BLUE,
            specular_color: Color::BLUE,
            shininess: 2.0
        })
    ];
}
