use core::time;
use std::time::{Duration, Instant};
use rand::Rng;

// loop constants
const UPDATE_INTERVAL: u32 = 40;

// debug constants
const DEBUG_LOOP: bool = false;
const DEBUG_TIME: bool = false;
const DEBUG_PARTICLES: bool = true;

// scene constants
const TIMESCALE: f32 = 1.0;
const GRAVITY: f32 = -9.8;
const PARTICLE_COUNT: i32 = 1;

// a struct made for physics particles
#[derive(Copy, Clone)]
struct Particle {
	location: [f32; 3],
	velocity: [f32; 3],
    acceleration: [f32; 3],
    gravity_enabled: bool
}

fn main() {
    let mut simulate: bool = true;
    
    // start the clock to keep track of real time
    let clock_start = Instant::now();

    // track the end time of the last loop
    let mut end_time = Instant::now();

    // keep track of the last tick time
    let mut last_tick = Instant::now();
    
    // a vector of particles
    let mut particles: Vec<Particle> = vec![];
    for i in 0..PARTICLE_COUNT {
        let new_particle = Particle {
            location: [0.0, 0.0, 10.0],
            velocity: [0.0; 3],
            acceleration: [0.0; 3],
            gravity_enabled: true
        };

        particles.push(new_particle)
    }

    while simulate {
        // update
        if delta_time(last_tick) >= UPDATE_INTERVAL {
            // DEBUG
            if DEBUG_TIME {
                println!("Real time: {}ms | Delta time: {}ms", delta_time(clock_start), delta_time(last_tick));
            }
            if DEBUG_PARTICLES {
                for i in 0..particles.len() {
                    println!("Time: {}ms | Delta time: {} | Particle {} Location: {:?}, Velocity: {:?}, Acceleration: {:?}", delta_time(clock_start), delta_time(last_tick), i, particles[i].location, particles[i].velocity, particles[i].acceleration);
                }
            }

            update(delta_time(last_tick), &mut particles);

            // record last tick time
            last_tick = Instant::now();
        }

        //display(delta_time, &test_particle);
    }
}

// update function
// TODO - I think I'm resetting delta time in the wrong place or just using it incorrectly; frameskips aren't happening at all
fn update(delta_time: u32, particles: &mut Vec<Particle>) {
    if DEBUG_LOOP {
        println!("Updating");
    }

    // calculate the exact timestep (fractional time in seconds) from delta time
    let timestep: f32 = delta_time as f32 / 1000.0;
    
    for particle in particles.iter_mut() {
        // add gravitational constant to instantaneous acceleration
        if particle.gravity_enabled {
            // acceleration += constant
            particle.acceleration[2] += GRAVITY;
        }
        
        // update velocity
        for i in 0..3 {
            // velocity += timestep * acceleration
            particle.velocity[i] = timestep.mul_add(particle.acceleration[i], particle.velocity[i]);

            // kill instantaneous acceleration
            particle.acceleration[i] = 0.0;
        }
        
        // update location
        for i in 0..3 {
            // location += timestep * velocity
            particle.location[i] = timestep.mul_add(particle.velocity[i], particle.location[i]);
        }
    
        // prevent the particle from going below the ground plane, and kill downward velocity upon "hitting" the ground plane
        if particle.location[2] <= 0.0 {
            particle.location[2] = 0.0;
            particle.velocity[2] = particle.velocity[2].max(0.0);
        }
    }
}

// render function
fn display(delta_time: Duration, particle: &Particle) {
    // calculate interpolation via delta time
    let interpolation: f32 = delta_time.as_millis() as f32 / UPDATE_INTERVAL as f32;

    // DEBUG
    if DEBUG_LOOP {
        println!("Displaying | Delta Time: {}ms | Relative Time: {}", delta_time.as_millis(), interpolation);
    }

    // DEBUG
    // println!("test_particle | Location: {:?} | Velocity: {:?}", render_particle.location, render_particle.velocity);
}

// 1D linear interpolation
fn lerp_1d(x: f64, y: f64, a: f64) -> f64 {
    x + ((y - x) * a)
}

// returns a vector3 with random components
fn make_vector3() -> [f32; 3] {
    let mut rng = rand::thread_rng();

    let mut vector3: [f32; 3] = [0.0; 3];

    for i in 0..3 {
        let component = rng.gen::<f32>();
        vector3[i] = component;
    }

    vector3
}

// gets the time in milliseconds that's elapsed since the earlier Instant
fn delta_time(earlier: Instant) -> u32 {
    Instant::now().duration_since(earlier).as_millis() as u32
}