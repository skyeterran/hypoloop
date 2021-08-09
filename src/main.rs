use std::time::{Duration, Instant};
const UPDATE_INTERVAL: u32 = 40;
const DEBUG_LOOP: bool = false;

// a struct made for physics objects
#[derive(Copy, Clone)]
struct PhysicsObject {
	location: [f32; 3],
	velocity: [f32; 3]
}

fn main() {
    // track the delta time (the duration of the previous loop in milliseconds)
    let mut delta_time = Duration::new(0, 0);

    // create a test physics object
    let mut test_object = PhysicsObject {
        location: [0.0, 0.0, 0.0],
        velocity: [0.0, 0.0, 0.0]
    };

    loop {
        // start timing the loop
        let start_time = Instant::now();

        // update
        if delta_time.as_millis() as u32 >= UPDATE_INTERVAL {
            if DEBUG_LOOP {
                println!("Updating");
            }
            update(delta_time, &mut test_object);

            // reset the delta time
            delta_time = Instant::now().duration_since(start_time);
        }

        // support interpolation via fractional "relative time"
        let relative_time: f32 = delta_time.as_millis() as f32 / UPDATE_INTERVAL as f32;

        if DEBUG_LOOP {
            println!("Displaying | Delta Time: {}ms | Relative Time: {}", delta_time.as_millis(), relative_time);
        }
        display(relative_time, &test_object);

        // update the time
        delta_time += Instant::now().duration_since(start_time);
    }
}

// update function
// TODO - I think I'm resetting delta time in the wrong place or just using it incorrectly; frameskips aren't happening at all
fn update(delta_time: Duration, object: &mut PhysicsObject) {
    // calculate the exact timestep (fractional time in seconds) from delta time
    let timestep: f32 = delta_time.as_millis() as f32 / 1000.0;
    println!("Timestep: {}", timestep);

    // gravity
    let acceleration: [f32; 3] = [0.0, 0.0, -9.8];

    // update velocity based on acceleration (specifically gravity)
    for i in 0..3 {
        object.velocity[i] += acceleration[i] * timestep;
    }

    // update location based on velocity
    for i in 0..3 {
        object.location[i] += object.velocity[i];
    }
}

// render function
fn display(interpolation: f32, object: &PhysicsObject) {
    // create a "render object", which we can apply interpolation to
    let mut render_object = *object;

    // interpolate physics values for smooth rendering
    for i in 0..3 {
        render_object.location[i] += render_object.velocity[i] * interpolation;
    }

    println!("Object location: {:?}", render_object.location);
}

// 1D linear interpolation
fn lerp_1d(x: f64, y: f64, a: f64) -> f64 {
    x + ((y - x) * a)
}