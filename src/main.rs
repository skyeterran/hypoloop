use std::time::Instant;
const UPDATE_INTERVAL: u32 = 40;
const MAX_FRAMESKIP: u32 = 5;

fn main() {
    // track the delta time (the duration of the previous loop in milliseconds)
    let mut delta_time: u32 = 0;

    // allow for interpolation

    loop {
        // start per-loop delay timer
        let timer = Instant::now();

        // track the number of update ticks
        let mut ticks: u32 = 0;

        update(&mut delta_time, &mut ticks);

        display();
        
        // update the delta time
        let elapsed_time: u32 = timer.elapsed().as_millis() as u32;
        delta_time += elapsed_time;
    }
}

// update function
// TODO - I think I'm resetting delta time in the wrong place or just using it incorrectly; frameskips aren't happening at all
fn update(delta_time: &mut u32, ticks: &mut u32) {
    // keep going even if frames aren't displaying, but halt if there are too many frameskips
    while *delta_time >= UPDATE_INTERVAL && *ticks < MAX_FRAMESKIP {
        println!("Updating...");
        waste_time(64);

        // record the tick
        *ticks += 1;
     
        // reset the delta time
        *delta_time = 0;
    }
}

// render function
fn display() {
    println!("Displaying...");
    waste_time(32);
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