use core::time;
use std::time::{Duration, Instant};

// sim constants
// UPDATE_INTERVAL is the minimum delay (in milliseconds) between update ticks
// TIMESCALE is the rate of the simulation proportional to real-time
// if REALTIME is false, the simulation runs as fast as possible and doesn't run the display function
const UPDATE_INTERVAL: u32 = 40;
const TIMESCALE: f32 = 1.0;
const REALTIME: bool = false;

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
    let mut irl_time = Duration::new(0, 0);
    let mut sim_time = Duration::new(0, 0);

    while simulate {
        // TODO - support frameskips
        if !REALTIME || delta_time(last_tick) >= UPDATE_INTERVAL {
            // mutable delta time and timescale for flexibility
            let mut current_timescale: f32;
            let mut current_delta_time: u32;
            
            // update clocks
            if REALTIME {
                current_timescale = TIMESCALE;
                current_delta_time = delta_time(last_tick);
                let elapsed_time = Instant::now().duration_since(last_tick);
                sim_time += elapsed_time.mul_f32(TIMESCALE);
                irl_time += elapsed_time;
            } else {
                current_timescale = 1.0;
                current_delta_time = UPDATE_INTERVAL;
                sim_time += Duration::from_millis(UPDATE_INTERVAL as u64);
                irl_time = Instant::now().duration_since(clock_start);
            }

            // DEBUG
            if DEBUG_TIME {
                println!("REALTIME: {} | IRL time: {}ms | Sim time: {}ms | Delta time: {}ms", REALTIME, irl_time.as_millis(), sim_time.as_millis(), current_delta_time);
            }

            // update
            update(current_delta_time, current_timescale);

            // record last tick time
            last_tick = Instant::now();
        }

        // display
        if REALTIME {
            display(delta_time(last_tick), TIMESCALE);
        }
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