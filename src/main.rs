mod core;
mod gfx;

use std::error::Error;
use winit::dpi::LogicalSize;
use winit::event::{Event, ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder};
use crate::gfx::vulkan::create_instance;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Thermite")
        .with_inner_size(LogicalSize::new(800,600))
        .build(&event_loop)?;

    log::debug!("Created window");
    log::debug!("Starting event loop");

    let instance = create_instance()?;
    instance.print_devices();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => { *control_flow = ControlFlow::Exit },
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        state: ElementState::Released, ..
                    }, ..
                }, ..
            } => { *control_flow = ControlFlow::Exit },
            _ => {}
        }
    });
}
