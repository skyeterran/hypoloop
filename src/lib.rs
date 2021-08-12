pub mod core {
    use std::time::{Duration, Instant};
    
    /// Contains mutable simulation state which can be changed via callback functions
    #[derive(Copy, Clone)]
    pub struct State {
        update_interval: u32,
        timescale: f32,
        simulate: bool,
        clock_start: Instant,
        last_tick: Instant,
        delta_time: u32,
        irl_time: Duration,
        sim_time: Duration
    }

    impl State {
        /// Creates a default State object
        pub fn new() -> State {
            // Create default state object
            let mut new_state = State {
                update_interval: 40,
                timescale: 1.0,
                simulate: true,
                clock_start: Instant::now(),
                last_tick: Instant::now(),
                delta_time: 0,
                irl_time: Duration::new(0,0),
                sim_time: Duration::new(0,0)
            };

            // Make sure that delta_time always starts the same as update_interval
            new_state.delta_time = new_state.update_interval;

            // Return this default state
            new_state
        }

        /// Returns the current "delta time", the time elapsed since the last update tick in milliseconds
        pub fn get_delta_time(self) -> u32 {
            self.delta_time
        }

        /// Returns the current IRL time elapsed since the start of the simulation
        pub fn get_irl_time(self) -> Duration {
            self.irl_time
        }

        /// Returns the current simulation time elapsed since the start of the simulation
        pub fn get_sim_time(self) -> Duration {
            self.sim_time
        }

        /// Returns the current "timescale", the speed of simulation time relative to real time
        pub fn get_timescale(self) -> f32 {
            self.timescale
        }

        /// Returns the time of the last tick
        pub fn get_last_tick(self) -> Instant {
            self.last_tick
        }

        /// Pauses the simulation from within update logic
        pub fn pause(&mut self) {
            self.simulate = false;
        }

        /// Resumes the simulation from within update logic
        pub fn resume(&mut self) {
            self.simulate = true;
        }

        /// Changes the simulation timescale
        pub fn set_timescale(&mut self, timescale: f32) {
            self.timescale = timescale;
        }

        /// Prints a string of information about the current time (IRL time, Sim time, Delta time (tick), Delta time (step))
        /// IRL time: Real time (in ms) elapsed since the start of the loop
        /// Sim time: Virtual time (in ms) elapsed since the start of the loop
        /// Delta time (tick): Real time (in ms) elapsed between the last tick and the previous tick
        /// Delta time (step): Real time (in ms with nanosecond accuracy) elapsed since the last update tick
        pub fn debug_time(self) {
            let elapsed_time = Instant::now().duration_since(self.last_tick);
            let loop_delay_ms = elapsed_time.as_nanos() as f32 / 1_000_000.0;
            println!("IRL time: {}ms | Sim time: {}ms | Delta time (tick): {}ms | Delta time (step): {}ms", self.irl_time.as_millis(), self.sim_time.as_millis(), self.delta_time, loop_delay_ms);
        }
    }

    /// The simulation loop itself
    pub struct Loop {
        state: State,
        realtime: bool
    }
    
    impl Loop {
        /// Creates a new simulation with default values
        pub fn new() -> Loop {
            // Return a Loop object with a default State
            Loop {
                state: State::new(),
                realtime: true
            }
        }
    
        /// Initializes or re-initializes the simulation
        pub fn init(&mut self) {
            // Make sure the simulation will run
            self.state.simulate = true;

            // reset the internal clocks
            self.state.clock_start = Instant::now();
            self.state.irl_time = Duration::new(0,0);
            self.state.sim_time = Duration::new(0,0);
        }

        /// Executes the per-loop logic (can be triggered manually so that hypoloop can be tied into external event loops)
        pub fn step(&mut self, mut update_callback: impl FnMut(&mut State), display_callback: impl Fn(&State)) {
            // don't run if the simulation is paused
            if self.state.simulate {
                // TODO - support frameskips
                if !self.realtime || delta_time(self.state.last_tick) >= self.state.update_interval {
                    // mutable delta time and timescale for flexibility
                    let elapsed_time = Instant::now().duration_since(self.state.last_tick);
                    
                    // update clocks
                    if self.realtime {
                        self.state.delta_time = delta_time(self.state.last_tick);
                        self.state.sim_time += elapsed_time.mul_f32(self.state.timescale);
                        self.state.irl_time += elapsed_time;
                    } else {
                        self.state.delta_time = self.state.update_interval;
                        self.state.sim_time += Duration::from_millis(self.state.update_interval as u64);
                        self.state.irl_time = Instant::now().duration_since(self.state.clock_start);
                    }
        
                    // update
                    update_callback(&mut self.state);
        
                    // record last tick time
                    self.state.last_tick = Instant::now();
                }
        
                // display
                if self.realtime {
                    display_callback(&self.state);
                }
            }
        }

        /// Turns real-time mode on/off
        pub fn set_realtime(&mut self, realtime: bool) {
            self.realtime = realtime;
        }
    }
    
    // gets the time in milliseconds that's elapsed since the earlier Instant
    fn delta_time(earlier: Instant) -> u32 {
        Instant::now().duration_since(earlier).as_millis() as u32
    }
}