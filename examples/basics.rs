use hypoloop::core::Loop;

// look into using closures for this
fn main() {
    // create sim and configure it
    let mut sim = Loop::new();
    //sim.set_realtime(false);

    // test variable
    let mut x: f32 = 0.0;

    // run the simulation using custom update logic
    sim.run(|state| {
        state.debug_tick();
        
        x += 2.0 * state.get_timescale();

        //println!("Delta time: {} | Timescale: {} | Sim time: {} | x: {}", state.get_delta_time(), state.get_timescale(), state.get_sim_time().as_millis(), x);
    });
}