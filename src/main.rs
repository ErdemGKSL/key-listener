pub mod models;
pub mod simulation;
pub mod direct;
pub mod complex;
pub mod hold_and_release;
pub mod mouse;

use crate::simulation::*;
use crate::direct::*;
use crate::complex::*;
use crate::hold_and_release::*;
use crate::mouse::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let handling_type = if args.len() > 1 {
        match args[1].to_uppercase().as_str() {
            "DIRECT" => 1,
            "COMPLEX" => 2,
            "HOLD_AND_RELEASE" => 3,
            "SIMULATION" => 4,
            "MOUSE" => 5,
            _ => {
                1
            }
        }
    } else {
        1
    };
    
    match handling_type {
        1 => direct_handling(),
        2 => complex_handling(),
        3 => hold_and_release_handling(),
        4 => key_simulation_handling(),
        5 => mouse_handling(),
        _ => direct_handling(),
    }
}