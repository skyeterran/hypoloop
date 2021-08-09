use std::time::Instant;
const UPDATE_INTERVAL: u32 = 80;

fn main() {
    // track the delta time (the duration of the previous loop in milliseconds)
    let mut delta_time: u32 = 0;

    // allow for interpolation

    loop {
        // start per-loop delay timer
        let timer = Instant::now();

        // update
        if delta_time >= UPDATE_INTERVAL {
            println!("Updating");
            update();

            // DEBUG

            // reset the delta time
            delta_time = timer.elapsed().as_millis() as u32;
        }

        // support interpolation via fractional "relative time"
        let relative_time: f32 = delta_time as f32 / UPDATE_INTERVAL as f32;

        println!("Displaying | Delta Time: {}ms | Relative Time: {}", delta_time, relative_time);
        display();

        // update the delta time
        let elapsed_time: u32 = timer.elapsed().as_millis() as u32;
        delta_time += elapsed_time;
    }
}

// update function
// TODO - I think I'm resetting delta time in the wrong place or just using it incorrectly; frameskips aren't happening at all
fn update() {
    waste_time(64);
}

// render function
fn display() {
    waste_time(4);
}

// 1D linear interpolation
fn lerp_1d(x: f64, y: f64, a: f64) -> f64 {
    x + ((y - x) * a)
}

// busywork function
fn waste_time(repeat: u32) {
    const BUFFER_RESOLUTION: usize = 512;
    
    // create an RGBA buffer and change every pixel one-by-one
    let mut rgba_buffer = vec![[[0.0f32; 4]; BUFFER_RESOLUTION]; BUFFER_RESOLUTION];

    // change every pixel in the buffer (repeat) times
    for _n in 0..repeat {
        for u in 0..BUFFER_RESOLUTION {
            for v in 0..BUFFER_RESOLUTION {
                rgba_buffer[u][v] = [1.0f32; 4];
            }
        }
    }
}