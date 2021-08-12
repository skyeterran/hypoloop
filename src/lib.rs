pub mod core {
    use std::time::{Duration, Instant};
    
    /// Contains mutable simulation state which can be changed via callback functions
    #[derive(Copy, Clone)]
    pub struct State {
        update_interval: u32,
        timescale: f32,
        simulate: bool,
        delta_time: u32,
        irl_time: Duration,
        sim_time: Duration,
        last_tick: Instant
    }

    impl State {
        /// Creates a default State object
        pub fn new() -> State {
            // Create default state object
            let mut new_state = State {
                update_interval: 40,
                timescale: 1.0,
                simulate: true,
                delta_time: 0,
                irl_time: Duration::new(0,0),
                sim_time: Duration::new(0,0),
                last_tick: Instant::now()
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

        /// Pauses the simulation
        pub fn pause(&mut self) {
            self.simulate = false;
        }

        /// Resumes the simulation
        pub fn resume(&mut self) {
            self.simulate = true;
        }

        /// Changes the simulation timescale
        pub fn set_timescale(&mut self, timescale: f32) {
            self.timescale = timescale;
        }

        /// Prints a string of information about the current tick
        pub fn debug_tick(self) {
            let elapsed_time = Instant::now().duration_since(self.last_tick);
            let loop_delay_ms = elapsed_time.as_nanos() as f32 / 1_000_000.0;
            let loop_rate_hz = 1000.0 / loop_delay_ms;
            println!("IRL time: {}ms | Sim time: {}ms | Tick delay/rate: {}ms/{}hz", self.irl_time.as_millis(), self.sim_time.as_millis(), loop_delay_ms, loop_rate_hz);
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
    
        /// Initializes and runs the simulation using a user-supplied callback as the update logic
        pub fn run(&mut self, mut update_callback: impl FnMut(&mut State)) {
            // Make sure the simulation will run
            self.state.simulate = true;

            // start the clock to keep track of real time
            let clock_start = Instant::now();
                
            while self.state.simulate {
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
                        self.state.irl_time = Instant::now().duration_since(clock_start);
                    }
        
                    // update
                    update_callback(&mut self.state);
        
                    // record last tick time
                    self.state.last_tick = Instant::now();
                }
        
                // display
                if self.realtime {
                    display(delta_time(self.state.last_tick), self.state.timescale, self.state.update_interval);
                }
            }
        }

        /// Turns real-time mode on/off
        pub fn set_realtime(&mut self, realtime: bool) {
            self.realtime = realtime;
        }
    }
    
    
    // update function
    // this is where all your per-tick logic should go
    fn update(user_function: fn(), delta_time: u32, timescale: f32) {
        // use timestep to scale per-tick calculations appropriately
        let timestep: f32 = delta_time as f32 / 1000.0 * timescale;
    
        // call user update function
        user_function();
    }
    
    // display function
    // this is where you should call a render function
    fn display(delta_time: u32, timescale: f32, update_interval: u32) {
        // use interpolation to smooth display values between ticks
        let interpolation: f32 = delta_time as f32 / update_interval as f32 * timescale;
    }
    
    // gets the time in milliseconds that's elapsed since the earlier Instant
    fn delta_time(earlier: Instant) -> u32 {
        Instant::now().duration_since(earlier).as_millis() as u32
    }
    
    // default update function (does nothing)
    fn default_update() {
    }
    
    // default display function (does nothing)
    fn default_display() {
    }
}