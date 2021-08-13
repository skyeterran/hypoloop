use std::time::Duration;
use hypoloop::core::{State, Loop};

// this is a demonstration of running two different Loops simultaneously, both acting on the same data
fn main() {
    // create a vector of sim loops
    let mut multiverse: Vec<&mut Loop> = vec![];

    // Sim A
    let mut sim_a = Loop::new();
    sim_a.set_update_interval(40);
    sim_a.mut_state().set_timescale(1.0);
    multiverse.push(&mut sim_a);
    
    // Sim B
    // twice the speed and twice the updates
    let mut sim_b = Loop::new();
    sim_b.set_update_interval(40);
    sim_b.mut_state().set_timescale(2.0);
    multiverse.push(&mut sim_b);
    
    // shared variable
    let mut x = Duration::new(0,0);

    // tick behavior
    // note how "state" switches between Sim A and B but x doesn't
    // I'm purposefully NOT moving ownership of x into this closure so that I can access it later
    let mut tick = |state: &mut State| {    
        // access loop metadata via the State object    
        x = (x + state.get_sim_time()) / 2;
        print!("Average sim time: {} | ", x.as_millis());

        // print information about the current tick's timings
        state.debug_time();
    };
    
    // create a closure containing your display logic
    let mut display = move |state: &mut State| {
        //
    };

    // run both sims simultaneously
    sim_a.init();
    sim_b.init();
    loop {
        sim_a.step(&mut tick, &mut display);
        sim_b.step(&mut tick, &mut display);
    }
}