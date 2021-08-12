use hypoloop::core::{State, Loop};

fn main() {
    // create a new sim loop
    let mut sim = Loop::new();
    sim.set_update_interval(20);

    // test variable
    let mut x: f32 = 0.0;

    // create a closure containing your update logic
    let mut update_logic = move |state: &mut State| {    
        // access loop metadata via the State object    
        x += state.get_timestep();
        print!("x: {} | ", x);

        // print information about the current tick's timings
        state.debug_time();
    };
    
    // create a closure containing your display logic
    let mut display_logic = move |state: &mut State| {
        //
    };

    // run the simulation with your user-defined update and display logic
    // initialize the sim (cleans internal clocks, etc.)
    sim.init();
    loop {
        // "step" the sim forward
        sim.step(&mut update_logic, &mut display_logic);
    }
}