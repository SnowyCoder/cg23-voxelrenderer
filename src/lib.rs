
use parser::Scene;
use wgpu::Instance;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopBuilder},
};

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

mod parser;
mod color;
mod app;
mod camera;
mod render;
mod model;
mod texture;

const MODEL: &'static str = include_str!("../models/christmas.vly");


fn run(event_loop: EventLoop<()>, initial_scene: Option<Scene>) {
    log::info!("Running mainloop...");

    // doesn't need to be re-considered later
    let instance = Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        //backends: wgpu::Backends::VULKAN,
        //backends: wgpu::Backends::GL,
        ..Default::default()
    });

    let mut app = app::App::new(instance);
    app.world_state.scene = initial_scene;

    // It's not recommended to use `run` on Android because it will call
    // `std::process::exit` when finished which will short-circuit any
    // Java lifecycle handling
    event_loop.run(move |event, event_loop| {
        log::debug!("Received Winit event: {event:?}");

        match event {
            Event::Resumed => {
                app.resume(event_loop);
            }
            Event::Suspended => {
                log::info!("Suspended, dropping render state...");
                app.render_state = None;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_size),
                ..
            } => {
                app.configure_surface_swapchain();
                // Winit: doesn't currently implicitly request a redraw
                // for a resize which may be required on some platforms...
                app.queue_redraw();
            }
            Event::WindowEvent{
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                app.world_state.camera_controller.update_camera(&mut app.world_state.camera);
                if let Some(rs) = app.render_state.as_mut() {
                    rs.camera_uniform.update_view_proj(&app.world_state.camera)
                }

                render::render(&mut app);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => event_loop.exit(),
            Event::WindowEvent { event, .. } => {
                if !app.world_state.camera_controller.process_events(&event) {
                    log::debug!("Window event {:#?}", event);
                }
            }
            _ => {}
        }

    }).expect("Event loop error");
}

#[allow(dead_code)]
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    use parser::parse_scene;
    use winit::platform::android::EventLoopBuilderExtAndroid;

    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    let scene = parse_scene(MODEL, None).expect("Invalid model provided");

    let event_loop = EventLoopBuilder::new().with_android_app(app).build().unwrap();
    run(event_loop, Some(scene));
}

#[allow(dead_code)]
#[cfg(not(target_os = "android"))]
fn main() {
    use std::{env, fs};

    use parser::parse_scene;

    env_logger::builder()
        .filter_level(log::LevelFilter::Info) // Default Log Level
        .parse_default_env()
        .init();

    let path = env::args().skip(1).next().expect("Must provide a model path");
    let file = fs::read_to_string(path).expect("Error reading model");

    let scene = parse_scene(&file, None).expect("Invalid model provided");

    //log::info!("{scene:?}");

    let event_loop = EventLoopBuilder::new().build().expect("Failed to get event loop");
    run(event_loop, Some(scene));
}
