#![deny(clippy::all)]
#![forbid(unsafe_code)]

use hypoloop::core::{State, Loop};
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: i16 = 64;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
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
    sim.set_update_interval(10);

    let mut update_logic = move |state: &mut State| {    
        // print information about the current tick's timings
        state.debug_time();
        world.update(state.get_delta_time(), state.get_timescale());
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
            box_x: 24,
            box_y: 16,
            velocity_x: 100,
            velocity_y: 100,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self, delta_time: u32, timescale: f32) {
        let timestep: f32 = delta_time as f32 / 1000.0 * timescale;

        if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += (self.velocity_x as f32 * timestep) as i16;
        self.box_y += (self.velocity_y as f32 * timestep) as i16;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let inside_the_box = x >= self.box_x
                && x < self.box_x + BOX_SIZE
                && y >= self.box_y
                && y < self.box_y + BOX_SIZE;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}