use std::sync::Arc;

use pixels::{Error, Pixels, SurfaceTexture};
use tao::dpi::LogicalSize;
use tao::event::{Event, KeyEvent, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::keyboard::KeyCode;
use tao::window::WindowBuilder;

use crate::Device;
use crate::ui::emulator_view::run_emulator;

pub const WIDTH: u32 = 160;
pub const HEIGHT: u32 = 144;

pub fn open_gamescreen(mut device: Device<'static>) -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let window = WindowBuilder::new()
            .with_title("Hello Pixels/Tao")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap();

        Arc::new(window)
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, Arc::clone(&window));
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;
        // device.screen = pixels.frame_mut().into();

        pixels
    };

    let _ = run_emulator(&mut device);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                // Close events
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::Escape,
                            ..
                        },
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }

                // Resize the window
                WindowEvent::Resized(size) => {
                    if let Err(_err) = pixels.resize_surface(size.width, size.height) {
                        *control_flow = ControlFlow::Exit;
                    }
                    window.request_redraw();
                }

                _ => {}
            },

            // Update internal state and request a redraw
            Event::MainEventsCleared => {
                // world.update();
                window.request_redraw();
            }

            // Draw the current frame
            Event::RedrawRequested(_) => {
                device.draw_screen(pixels.frame_mut());

                if let Err(_) = pixels.render() {
                    *control_flow = ControlFlow::Exit;
                }
            }

            _ => {
                // Handle menu events
            }
        }
    });
}
