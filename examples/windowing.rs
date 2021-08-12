use hypoloop::core::{State, Loop};
use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{CursorIcon, WindowBuilder},
};

fn main() {
    // create sim with default configuration
    let mut sim = Loop::new();

    // test variable
    let mut x: f32 = 0.0;
    
    // create a winit event loop
    let event_loop = EventLoop::new();
    
    // create a winit window
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Windowing test with hypoloop");

    // create a closure containing your update logic
    let mut update_logic = move |state: &mut State| {    
        // access loop metadata via the State object    
        x += state.get_timescale();
        print!("x: {} | ", x);

        // print information about the current tick's timings
        state.debug_time();
    };
    
    // create a closure containing your display logic
    let display_logic = move |state: &mut State| {
        // redraw the winit window
        window.request_redraw();
    };
    
    // initialize the sim (cleans internal clocks, etc.)
    sim.init();

    // run the winit event loop with embedded hypoloop sim
    event_loop.run(move |event, _, control_flow| {
        // "step" the sim forward
        sim.step(&mut update_logic, &mut display_logic);
    });
}