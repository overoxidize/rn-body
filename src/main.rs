#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
)]
use std::mem;
use std::arch::x86_64::*;
use std::f64::consts::PI;
fn main() {

    println!("Hello, world!");
}


#[repr(C)]
struct body {
    position: [f64; 3],
    velocity: [f64; 3],
    mass: f64
}

const SOLAR_MASS: f64 = 4. * PI * PI;
const DAYS_PER_YEAR: f64 = 365.24;
const BODIES_COUNT: usize = 5;

static mut solar_bodies: [body; BODIES_COUNT] = [
    body { 
        mass: SOLAR_MASS,
        position: [0.; 3],
        velocity: [0.; 3],
    },
    body {// Jupiter
        position: [
            4.84143144246472090e+00,
            -1.16032004402742839e+00,
            -1.03622044471123109e-01
        ],
        velocity: [
            1.66007664274403694e-03 * DAYS_PER_YEAR,
             7.69901118419740425e-03 * DAYS_PER_YEAR,
            -6.90460016972063023e-05 * DAYS_PER_YEAR
        ],
        mass: 9.54791938424326609e-04 * SOLAR_MASS
    }, 
    body { // Saturn
        position: [
            8.34336671824457987e+00,
             4.12479856412430479e+00,
            -4.03523417114321381e-01
        ],
        velocity: [
            -2.76742510726862411e-03 * DAYS_PER_YEAR,
             4.99852801234917238e-03 * DAYS_PER_YEAR,
             2.30417297573763929e-05 * DAYS_PER_YEAR
        ],
        mass:  2.85885980666130812e-04 * SOLAR_MASS
    },

    body { // Uranus 
        position: [
            1.28943695621391310e+01,
            -1.51111514016986312e+01,
            -2.23307578892655734e-01
        ],
        velocity: [
            2.96460137564761618e-03 * DAYS_PER_YEAR,
             2.37847173959480950e-03 * DAYS_PER_YEAR,
            -2.96589568540237556e-05 * DAYS_PER_YEAR
        ],
        mass: 4.36624404335156298e-05 * SOLAR_MASS
    },

    body { // Neptune
        position: [
            1.53796971148509165e+01,
            -2.59193146099879641e+01,
             1.79258772950371181e-01
        ],
        velocity: [
            2.68067772490389322e-03 * DAYS_PER_YEAR,
             1.62824170038242295e-03 * DAYS_PER_YEAR,
            -9.51592254519715870e-05 * DAYS_PER_YEAR
        ],
        mass: 5.15138902046611451e-05 * SOLAR_MASS
    }
];

unsafe fn offset_momentum(bodies: *mut body) {

    for i in 0..BODIES_COUNT {
        for m in 0..3 {
            
            (*bodies.add(0)).velocity[m] 

            //# `*bodies.add(0)` 
                // dereferences the value stored in the
                // bodies param, which is a raw mutable pointer,
                // then the .add() function, which is syntactic sugar over
                // pointer arithmetic to avoid overloading,
                // calculates the offset from a pointer, where the
                // offset is count * the size of the type (in bytes) being stored in the memory address
                // aimed at by the pointer.
                // Now that we have access to bodies, we can index into the
                // velocity array, with the variable m.

                -= (*bodies.add(i)).velocity[m]

                    //# `-= *bodies.add(i).velocity[m]`
                        // Repeats the above process, except we're ~subtracting
                        // from the above value, the value we gain from
                        // offsetting the pointer by i * the size of the type,
                        // and since `i` ranges over 0-4, for each of our bodies,
                        // we will offset all of the velocities as necessary.
                    //#~ `* (*bodies.add(i)).mass / SOLAR_MASS
                        // Before we do the subtraction, we take the product of our subtrahend, the value of *bodies.add(i).velocity[m]), and
                        // the result of dividing the given bodies mass, by the solar_mass, in order to offset the effect of the movement of other bodies on the sun, so that our n-body system,
                        // matches expectations from physical observation.
                    * (*bodies.add(i)).mass / SOLAR_MASS
        }
    }


}

unsafe fn output_energy(bodies: *mut body) {
    let mut energy = 0.;

    for i in 0..BODIES_COUNT {
        // Adding the kinetic energy of the bodies in the system.
            // Noting that the kinetic energy of an object is directly proportional
            // to the the square of the velocity, i.e K.E = 1/2 * (mv^2).
        energy += 0.5 * (*bodies.add(i)).mass * (
                (*bodies.add(i)).velocity[0] * (*bodies.add(i)).velocity[0] +
                (*bodies.add(i)).velocity[1] * (*bodies.add(i)).velocity[1] +
                (*bodies.add(i)).velocity[2] * (*bodies.add(i)).velocity[2] 
            );

        for j in i+1..BODIES_COUNT {
            // Sums up the potential energy betweeen a given body in the
            // outer loop, and every other possible body.
            let mut position_delta = [mem::MaybeUninit::<f64>::uninit(); 3];
            // MaybeUninit<T> is for expressing addresses which have not been initialized to a value.
            for m in 0..3 {
                position_delta[m].as_mut_ptr().write(
                    // Here, we write to a raw pointer, without reading from it first.
                    (*bodies.add(i)).position[m] 
                            - (*bodies.add(j)).position[m]
                );
            }
            
            let position_delta: [f64; 3] = mem::transmute((position_delta));
            // mem::transmute allows for converting between types as long as they're
            // represented by the same number of bits (or bytes, possibly), in memory.
            energy -= (*bodies.add(i)).mass
            * (*bodies.add(j)).mass
            / f64::sqrt(
                position_delta[0] * position_delta[0] +
                    position_delta[1] * position_delta[1] +
                    position_delta[2] * position_delta[2]
            );


        }
    }

    println!("{:.9}", energy);
}