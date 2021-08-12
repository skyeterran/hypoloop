use hypoloop::core::{State, Loop};

// look into using closures for this
fn main() {
    // create sim and configure it
    let mut sim = Loop::new();

    // test variable
    let mut x: f32 = 0.0;

    let update_logic = |state: &mut State| {        
        x += state.get_timescale();
        print!("x: {} | ", x);

        state.debug_tick();
    };

    let display_logic = |state: &State| {
        // put all display logic here
    };

    // run the simulation using custom update and display logic
    sim.run(update_logic, display_logic);
}