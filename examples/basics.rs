use hypoloop::core::{State, Loop};

fn main() {
    // create sim with default configuration
    let mut sim = Loop::new();

    // test variable
    let mut x: f32 = 0.0;

    // create a closure containing your update logic
    let mut update_logic = move |state: &mut State| {    
        // access loop metadata via the State object    
        x += state.get_timescale();
        print!("x: {} | ", x);

        // print information about the current tick's timings
        state.debug_time();
    };
    
    // create a closure containing your display logic
    let display_logic = move |state: &State| {
        //
    };

    // run the simulation with your user-defined update and display logic
    // initialize the sim (cleans internal clocks, etc.)
    sim.init();
    loop {
        // "step" the sim forward
        sim.step(&mut update_logic, &display_logic);
    }
}