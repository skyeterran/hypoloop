use hypoloop::Simulation;

// look into using closures for this
fn main() {
    // create sim and configure it
    let mut sim = Simulation::new();
    sim.set_update_function(test);
    //sim.set_realtime(false);

    // run sim
    sim.run();
}

fn test() {
    println!("Test");
}