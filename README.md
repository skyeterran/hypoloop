# hypoloop
## A flexible game-like loop for real-time simulation and rendering
### Features:
- Constant update rate
- Variable display rate
- Arbitrary simulation timescale
- Real-time can be disabled for high-speed simulations

## Example
```rust
use hypoloop::core::{State, Loop};

fn main() {
    // create sim with default configuration
    let mut sim = Loop::new();

    // test variable
    let mut x: f32 = 0.0;

    // create a closure containing your update logic
    let update_logic = |state: &mut State| {    
        // access loop metadata via the State object    
        x += state.get_timescale();
        print!("x: {} | ", x);

        // print information about the current tick's timings
        state.debug_tick();
    };

    // create a closure containing your display logic
    let display_logic = |state: &State| {
        //
    };

    // run the simulation with your user-defined update and display logic
    sim.run(update_logic, display_logic);
}
```
