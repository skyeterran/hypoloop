use core::time;
use std::time::{Duration, Instant};

// sim constants
// update interval is the minimum delay (in milliseconds) between update ticks
const UPDATE_INTERVAL: u32 = 40;
// timescale is the rate of the simulation proportional to real-time
const TIMESCALE: f32 = 1.0;

// debug constants
const DEBUG_LOOP: bool = false;
const DEBUG_TIME: bool = true;

fn main() {
    // allow the simulation to be stopped from within the loop (by setting simulate to false)
    let mut simulate: bool = true;
    
    // start the clock to keep track of real time
    let clock_start = Instant::now();

    // keep track of the last tick time
    let mut last_tick = Instant::now();

    // real-time and sim-time clocks
    let mut real_time = Duration::new(0, 0);
    let mut sim_time = Duration::new(0, 0);

    while simulate {
        // TODO - support frameskips
        if delta_time(last_tick) >= UPDATE_INTERVAL {
            // update clocks
            let elapsed_time = Instant::now().duration_since(last_tick);
            real_time += elapsed_time;
            sim_time += elapsed_time.mul_f32(TIMESCALE);

            // DEBUG
            if DEBUG_TIME {
                println!("Real time: {}ms | Sim time: {}ms | Delta time: {}ms", real_time.as_millis(), sim_time.as_millis(), delta_time(last_tick));
            }

            // update
            update(delta_time(last_tick), TIMESCALE);

            // record last tick time
            last_tick = Instant::now();
        }

        // display
        display(delta_time(last_tick), TIMESCALE);
    }
}

// update function
// this is where all your per-tick logic should go
fn update(delta_time: u32, timescale: f32) {
    // DEBUG
    if DEBUG_LOOP {
        println!("Updating...");
    }

    // use timestep to scale per-tick calculations appropriately
    let timestep: f32 = delta_time as f32 / 1000.0 * timescale;
}

// display function
// this is where you should call a render function
fn display(delta_time: u32, timescale: f32) {
    // DEBUG
    if DEBUG_LOOP {
        println!("Displaying...");
    }

    // use interpolation to smooth display values between ticks
    let interpolation: f32 = delta_time as f32 / UPDATE_INTERVAL as f32 * timescale;
}

// gets the time in milliseconds that's elapsed since the earlier Instant
fn delta_time(earlier: Instant) -> u32 {
    Instant::now().duration_since(earlier).as_millis() as u32
}