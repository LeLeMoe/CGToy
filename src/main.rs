use std::time;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod pipeline;
mod render;

#[tokio::main]
async fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("CGToy")
        .build(&event_loop)
        .unwrap();
    let mut pipeline_state = pipeline::PipelineState::new(&window).await;
    let mut time_last = time::Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { window_id, event } => {
                if window_id == window.id() {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(new_size) => pipeline_state.resize(new_size),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            pipeline_state.resize(*new_inner_size)
                        }
                        _ => (),
                    }
                }
            }
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(_) => {
                pipeline_state.render();
                // Calculate fps
                let time_now = time::Instant::now();
                let render_time = time_now - time_last;
                let fps = 1_000_000 / render_time.as_micros();
                window.set_title(format!("CGToy - fps:{}", fps).as_str());
                time_last = time_now;
            }
            _ => (),
        }
    });
}
