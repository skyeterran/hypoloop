use std::time::{Duration, Instant};

// loop constants
const UPDATE_INTERVAL: u32 = 40;
const DEBUG_LOOP: bool = false;

// physics constants
const GRAVITY: f32 = -9.8;

// a struct made for physics objects
#[derive(Copy, Clone)]
struct PhysicsObject {
	location: [f32; 3],
	velocity: [f32; 3],
    gravity_enabled: bool
}

fn main() {
    let mut simulate: bool = true;

    // global time
    let mut time = Duration::new(0,0);

    // track the delta time (the duration of the previous loop in milliseconds)
    let mut delta_time = Duration::new(0, 0);

    // create a test physics object
    let mut test_object = PhysicsObject {
        location: [0.0, 0.0, 10.0],
        velocity: [0.0, 5.0, 0.0],
        gravity_enabled: true
    };

    while simulate {
        // start timing the loop
        let start_time = Instant::now();

        // update
        if delta_time.as_millis() as u32 >= UPDATE_INTERVAL {
            update(delta_time, &mut test_object);
            
            // reset the delta time
            delta_time = Duration::new(0,0);
        }
        
        // DEBUG
        println!("Time: {}s | test_object | Location: {:?} | Velocity: {:?}", time.as_millis() as f32 / 1000.0, test_object.location, test_object.velocity);
        
        //display(delta_time, &test_object);
        
        // update the time
        let end_time = Instant::now().duration_since(start_time);
        delta_time += end_time;
        time += end_time;
    }
}

// update function
// TODO - I think I'm resetting delta time in the wrong place or just using it incorrectly; frameskips aren't happening at all
fn update(delta_time: Duration, object: &mut PhysicsObject) {
    if DEBUG_LOOP {
        println!("Updating");
    }

    // calculate the exact timestep (fractional time in seconds) from delta time
    let timestep: f32 = delta_time.as_millis() as f32 / 1000.0;
    
    // update location based on velocity
    for i in 0..3 {
        object.location[i] += timestep * object.velocity[i];
    }

    // gravity
    if object.gravity_enabled {
        object.velocity[2] += GRAVITY * timestep;
    }

    // prevent the object from going below the ground plane, and kill downward velocity upon "hitting" the ground plane
    if object.location[2] <= 0.0 {
        object.location[2] = 0.0;
        object.velocity[2] = object.velocity[2].max(0.0);
    }
}

// render function
fn display(delta_time: Duration, object: &PhysicsObject) {
    // calculate interpolation via delta time
    let interpolation: f32 = delta_time.as_millis() as f32 / UPDATE_INTERVAL as f32;

    // DEBUG
    if DEBUG_LOOP {
        println!("Displaying | Delta Time: {}ms | Relative Time: {}", delta_time.as_millis(), interpolation);
    }

    // create a "render object", which we can apply interpolation to
    let mut render_object = *object;

    // interpolate physics values for smooth rendering
    for i in 0..3 {
        render_object.location[i] += render_object.velocity[i] * interpolation;
    }

    // DEBUG
    // println!("test_object | Location: {:?} | Velocity: {:?}", render_object.location, render_object.velocity);
}

// 1D linear interpolation
fn lerp_1d(x: f64, y: f64, a: f64) -> f64 {
    x + ((y - x) * a)
}