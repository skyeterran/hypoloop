use std::time::{Duration, Instant};

// debug constants
const DEBUG_LOOP: bool = false;
const DEBUG_TIME: bool = true;

/// Contains all per-simulation logic and state
// update_interval is the minimum delay (in milliseconds) between update ticks
// timescale is the rate of the simulation proportional to real-time
// if realtime is false, the simulation runs as fast as possible and doesn't run the display function
pub struct Simulation {
    update_interval: u32,
    timescale: f32,
    realtime: bool,
    simulate: bool
}

impl Simulation {
    // Creates a new simulation with default values
    pub fn new() -> Simulation {
        Simulation {
            update_interval: 40,
            timescale: 1.0,
            realtime: true,
            simulate: true
        }
    }

    /// Initializes and runs the simulation
    pub fn run(&self) {
        // start the clock to keep track of real time
        let clock_start = Instant::now();
    
        // keep track of the last tick time
        let mut last_tick = Instant::now();
    
        // real-time and sim-time clocks
        let mut irl_time = Duration::new(0, 0);
        let mut sim_time = Duration::new(0, 0);
    
        while self.simulate {
            // TODO - support frameskips
            if !self.realtime || delta_time(last_tick) >= self.update_interval {
                // mutable delta time and timescale for flexibility
                let mut current_timescale: f32;
                let mut current_delta_time: u32;
                let elapsed_time = Instant::now().duration_since(last_tick);
                
                // update clocks
                if self.realtime {
                    current_timescale = self.timescale;
                    current_delta_time = delta_time(last_tick);
                    sim_time += elapsed_time.mul_f32(self.timescale);
                    irl_time += elapsed_time;
                } else {
                    current_timescale = 1.0;
                    current_delta_time = self.update_interval;
                    sim_time += Duration::from_millis(self.update_interval as u64);
                    irl_time = Instant::now().duration_since(clock_start);
                }
    
                // DEBUG
                if DEBUG_TIME {
                    let loop_delay_ms = elapsed_time.as_nanos() as f32 / 1_000_000.0;
                    let loop_rate_hz = 1000.0 / loop_delay_ms;
                    println!("Realtime: {} | IRL time: {}ms | Sim time: {}ms | Tick delay/rate: {}ms/{}hz", self.realtime, irl_time.as_millis(), sim_time.as_millis(), loop_delay_ms, loop_rate_hz);
                }
    
                // update
                update(current_delta_time, current_timescale);
    
                // record last tick time
                last_tick = Instant::now();
            }
    
            // display
            if self.realtime {
                display(delta_time(last_tick), self.timescale, self.update_interval);
            }
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
fn display(delta_time: u32, timescale: f32, update_interval: u32) {
    // DEBUG
    if DEBUG_LOOP {
        println!("Displaying...");
    }

    // use interpolation to smooth display values between ticks
    let interpolation: f32 = delta_time as f32 / update_interval as f32 * timescale;
}

// gets the time in milliseconds that's elapsed since the earlier Instant
fn delta_time(earlier: Instant) -> u32 {
    Instant::now().duration_since(earlier).as_millis() as u32
}