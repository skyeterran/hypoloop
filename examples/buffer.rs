#![deny(clippy::all)]
#![forbid(unsafe_code)]

use hypoloop::core::{State, Loop};
use rand::Rng;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const BOX_SIZE: i16 = 64;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    target: [i16; 2]
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
        .with_title("Hello Pixels")
        .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
        };
        
        let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = World::new();
    
    // create sim with default configuration
    let mut sim = Loop::new();
    //sim.set_update_interval(10);

    let mut update_logic = move |state: &mut State| {    
        // print information about the current tick's timings
        state.debug_time();
        world.update(state.get_timestep());
        world.draw(pixels.get_frame());
        if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                state.pause();
                return;
            }
    };

    // create a closure containing your display logic
    let mut display_logic = move |state: &mut State| {
        // Draw the current frame
        window.request_redraw();
    };

    sim.init();
    event_loop.run(move |event, _, control_flow| {
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // step the sim forward
        sim.step(&mut update_logic, &mut display_logic);
    });
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            target: [0, 0]
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self, timestep: f32) {
        let speed: f32 = 500.0;

        let mut new_target = self.target;

        // update the target
        if new_target[0] < WIDTH as i16 {
            new_target[0] += (speed * timestep) as i16;
        } else {
            new_target[0] = 0;
        }

        self.target = new_target;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;
            
            let mut old_pixel = [0u8; 4];
            for j in 0..4 {
                // get the old pixel and decay it
                old_pixel[j] = pixel[j];
                if old_pixel[j] > 0 {
                    old_pixel[j] -= 1;
                }
            }

            let condition = x <= self.target[0];

            let rgba = if condition {
                [0xff, 0x00, 0x00, 0xff]
            } else {
                old_pixel
            };
            
            pixel.copy_from_slice(&rgba);
        }
    }
}