pub mod models;

#[cfg(feature = "simulation")]
pub mod simulation;
#[cfg(feature = "direct")]
pub mod direct;
#[cfg(feature = "complex")]
pub mod complex;
#[cfg(feature = "hold_and_release")]
pub mod hold_and_release;
#[cfg(feature = "mouse")]
pub mod mouse;

#[cfg(feature = "simulation")]
use crate::simulation::*;
#[cfg(feature = "direct")]
use crate::direct::*;
#[cfg(feature = "complex")]
use crate::complex::*;
#[cfg(feature = "hold_and_release")]
use crate::hold_and_release::*;
#[cfg(feature = "mouse")]
use crate::mouse::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let handling_type = if args.len() > 1 {
        match args[1].to_uppercase().as_str() {
            #[cfg(feature = "direct")]
            "DIRECT" => 1,
            #[cfg(feature = "complex")]
            "COMPLEX" => 2,
            #[cfg(feature = "hold_and_release")]
            "HOLD_AND_RELEASE" => 3,
            #[cfg(feature = "simulation")]
            "SIMULATION" => 4,
            #[cfg(feature = "mouse")]
            "MOUSE" => 5,
            _ => {
                #[cfg(feature = "direct")]
                { 1 }
                #[cfg(all(not(feature = "direct"), feature = "complex"))]
                { 2 }
                #[cfg(all(not(feature = "direct"), not(feature = "complex"), feature = "hold_and_release"))]
                { 3 }
                #[cfg(all(not(feature = "direct"), not(feature = "complex"), not(feature = "hold_and_release"), feature = "simulation"))]
                { 4 }
                #[cfg(all(not(feature = "direct"), not(feature = "complex"), not(feature = "hold_and_release"), not(feature = "simulation"), feature = "mouse"))]
                { 5 }
                #[cfg(not(any(feature = "direct", feature = "complex", feature = "hold_and_release", feature = "simulation", feature = "mouse")))]
                { panic!("No features enabled!") }
            }
        }
    } else {
        #[cfg(feature = "direct")]
        { 1 }
        #[cfg(all(not(feature = "direct"), feature = "complex"))]
        { 2 }
        #[cfg(all(not(feature = "direct"), not(feature = "complex"), feature = "hold_and_release"))]
        { 3 }
        #[cfg(all(not(feature = "direct"), not(feature = "complex"), not(feature = "hold_and_release"), feature = "simulation"))]
        { 4 }
        #[cfg(all(not(feature = "direct"), not(feature = "complex"), not(feature = "hold_and_release"), not(feature = "simulation"), feature = "mouse"))]
        { 5 }
        #[cfg(not(any(feature = "direct", feature = "complex", feature = "hold_and_release", feature = "simulation", feature = "mouse")))]
        { panic!("No features enabled!") }
    };
    
    match handling_type {
        #[cfg(feature = "direct")]
        1 => direct_handling(),
        #[cfg(feature = "complex")]
        2 => complex_handling(),
        #[cfg(feature = "hold_and_release")]
        3 => hold_and_release_handling(),
        #[cfg(feature = "simulation")]
        4 => key_simulation_handling(),
        #[cfg(feature = "mouse")]
        5 => mouse_handling(),
        _ => panic!("Invalid handling type"),
    }
}