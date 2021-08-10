use std::time::{Duration, Instant};
use rand::Rng;

// loop constants
const UPDATE_INTERVAL: u32 = 40;

// debug constants
const DEBUG_LOOP: bool = false;
const DEBUG_OBJECTS: bool = false;
const DEBUG_PERFORMANCE: bool = true;
const FAIL_DELAY: u32 = 50;

// scene constants
const GRAVITY: f32 = -9.8;
const OBJECT_COUNT: i32 = 100;

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
    
    // a vector of objects to calculate physics for
    let mut objects: Vec<PhysicsObject> = vec![];
    let mut rng = rand::thread_rng();
    for i in 0..OBJECT_COUNT {
        let new_object = PhysicsObject {
            location: [0.0, 0.0, 0.0],
            velocity: [rng.gen::<f32>() * 10.0, rng.gen::<f32>() * 10.0, rng.gen::<f32>() * 10.0],
            gravity_enabled: true
        };

        objects.push(new_object)
    }

    while simulate {
        // start timing the loop
        let start_time = Instant::now();

        // update
        if delta_time.as_millis() as u32 >= UPDATE_INTERVAL {
            update(delta_time, &mut objects);
            
            // DEBUG
            if DEBUG_PERFORMANCE {
                let interval = delta_time.as_millis() as u32;
                println!("Update interval: {}ms | Object count: {}", interval, objects.len());

                // add more objects until it slows down enough
                if interval <= UPDATE_INTERVAL {
                    for i in 0..10000 {
                        let new_object = PhysicsObject {
                            location: [0.0, 0.0, 0.0],
                            velocity: [rng.gen::<f32>() * 10.0, rng.gen::<f32>() * 10.0, rng.gen::<f32>() * 10.0],
                            gravity_enabled: true
                        };
                
                        objects.push(new_object)
                    }
                } else {
                    // stop the simulation
                    simulate = false;
                }
            }

            // reset the delta time
            delta_time = Duration::new(0,0);
        }
        
        //display(delta_time, &test_object);
        
        // update the time
        let end_time = Instant::now().duration_since(start_time);
        delta_time += end_time;
        time += end_time;
                
        // DEBUG
        if DEBUG_OBJECTS {
            for object in objects.iter() {
                println!("Time: {}s | Location: {:?} | Velocity: {:?}", time.as_millis() as f32 / 1000.0, object.location, object.velocity);
            }
        }
    }
}

// update function
// TODO - I think I'm resetting delta time in the wrong place or just using it incorrectly; frameskips aren't happening at all
fn update(delta_time: Duration, objects: &mut Vec<PhysicsObject>) {
    if DEBUG_LOOP {
        println!("Updating");
    }

    // calculate the exact timestep (fractional time in seconds) from delta time
    let timestep: f32 = delta_time.as_millis() as f32 / 1000.0;
    
    for object in objects.iter_mut() {
        // update location based on velocity
        for i in 0..3 {
            // location += timestep * velocity
            // use an FMA here to halve the rounding error
            object.location[i] = timestep.mul_add(object.velocity[i], object.location[i]);
        }
    
        // gravity
        if object.gravity_enabled {
            // velocity += timestep * gravity
            // use an FMA here to halve the rounding error
            object.velocity[2] = timestep.mul_add(GRAVITY, object.velocity[2]);
        }
    
        // prevent the object from going below the ground plane, and kill downward velocity upon "hitting" the ground plane
        if object.location[2] <= 0.0 {
            object.location[2] = 0.0;
            object.velocity[2] = object.velocity[2].max(0.0);
        }
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

    // DEBUG
    // println!("test_object | Location: {:?} | Velocity: {:?}", render_object.location, render_object.velocity);
}

// 1D linear interpolation
fn lerp_1d(x: f64, y: f64, a: f64) -> f64 {
    x + ((y - x) * a)
}