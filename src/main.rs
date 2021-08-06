use std::time::Instant;
const UPDATE_INTERVAL: u32 = 40;

fn main() {
    // track the update delay's exponential moving average (EMA)
    let mut frame_delay_ema: f64 = 0.0;

    // track the latest delays
    let mut frame_delay: u32 = 0;
    let mut tick_delay: u32 = 0;
    
    loop {
        // start per-loop delay timer
        let timer = Instant::now();

        // DEBUG - track if a tick has happened on this frame
        let mut ticked: bool = false;

        // update (game logic, etc.)
        // executes every UPDATE_INTERVAL milliseconds at maximum
        if tick_delay >= UPDATE_INTERVAL {
            // perform all per-tick logic here
            update();

            // reset the tick delay
            tick_delay = 0;

            // DEBUG
            ticked = true;
        }
        
        // display (rendering)
        // executes as fast as possible
        {
            display();
        }
        
        // get the current frame delay, update the frame delay EMA and the tick delay
        frame_delay = timer.elapsed().as_millis() as u32;
        frame_delay_ema = lerp_1d(frame_delay_ema, frame_delay as f64, 0.1f64);
        tick_delay += frame_delay;
        
        // debug
        let frame_rate = (1000.0f64 / frame_delay_ema as f64) as u32;
        if ticked {
            println!("Frame Delay: {}ms | Frame Rate: {}Hz | Ticked!", frame_delay, frame_rate);
        } else {
            println!("Frame Delay: {}ms | Frame Rate: {}Hz", frame_delay, frame_rate);
        }
    }
}

// update function
fn update() {
    waste_time(64);
}

// render function
fn display() {
    waste_time(16);
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