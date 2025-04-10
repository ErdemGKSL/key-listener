use device_query::{DeviceQuery, DeviceState, Keycode};
use serde::Serialize;
use std::thread;
use std::time::Duration;
use std::env;

#[derive(Serialize)]
struct KeyEvent {
    key: String,
    pressed: bool,
    timestamp: u64,
}

#[derive(Serialize)]
struct ComplexKeyEvent {
    keys: Vec<String>,
    event_type: String,
    timestamp: u64,
}

#[derive(Serialize)]
struct KeySequenceEvent {
    keys_pressed: Vec<String>,
    start_time: u64,
    end_time: u64,
    duration_ms: u64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let handling_type = if args.len() > 1 {
        match args[1].to_uppercase().as_str() {
            "DIRECT" => 1,
            "COMPLEX" => 2,
            "HOLD_AND_RELEASE" => 3,
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
        _ => direct_handling(),
    }
}

fn direct_handling() {
    let device_state = DeviceState::new();
    let mut previous_keys: Vec<Keycode> = Vec::new();
    
    loop {
        let keys = device_state.get_keys();
        
        for key in &keys {
            if !previous_keys.contains(key) {
                let event = KeyEvent {
                    key: format!("{:?}", key),
                    pressed: true,
                    timestamp: chrono::Utc::now().timestamp_millis() as u64,
                };
                println!("{}", serde_json::to_string(&event).unwrap());
            }
        }
        
        for key in &previous_keys {
            if !keys.contains(key) {
                let event = KeyEvent {
                    key: format!("{:?}", key),
                    pressed: false,
                    timestamp: chrono::Utc::now().timestamp_millis() as u64,
                };
                println!("{}", serde_json::to_string(&event).unwrap());
            }
        }
        
        previous_keys = keys;
        thread::sleep(Duration::from_millis(10));
    }
}

fn complex_handling() {
    let device_state = DeviceState::new();
    let mut previous_keys: Vec<Keycode> = Vec::new();
    let mut current_combination: Vec<String> = Vec::new();
    
    loop {
        let keys = device_state.get_keys();
        
        if keys != previous_keys {
            for key in &keys {
                if !previous_keys.contains(key) {
                    let key_str = format!("{:?}", key);
                    
                    if !current_combination.contains(&key_str) {
                        current_combination.push(key_str);
                    }
                    
                    let event = KeyEvent {
                        key: format!("{:?}", key),
                        pressed: true,
                        timestamp: chrono::Utc::now().timestamp_millis() as u64,
                    };
                    println!("{}", serde_json::to_string(&event).unwrap());
                }
            }
            
            for key in &previous_keys {
                if !keys.contains(key) {
                    let key_str = format!("{:?}", key);
                    
                    if let Some(index) = current_combination.iter().position(|k| k == &key_str) {
                        current_combination.remove(index);
                    }
                    
                    let event = KeyEvent {
                        key: format!("{:?}", key),
                        pressed: false,
                        timestamp: chrono::Utc::now().timestamp_millis() as u64,
                    };
                    println!("{}", serde_json::to_string(&event).unwrap());
                }
            }
            
            if !keys.is_empty() {
                let complex_event = ComplexKeyEvent {
                    keys: current_combination.clone(),
                    event_type: if previous_keys.is_empty() { "press".to_string() } 
                              else if keys.is_empty() { "release".to_string() } 
                              else { "combination".to_string() },
                    timestamp: chrono::Utc::now().timestamp_millis() as u64,
                };
                println!("{}", serde_json::to_string(&complex_event).unwrap());
            }
            
            previous_keys = keys;
        }
        
        thread::sleep(Duration::from_millis(10));
    }
}

fn hold_and_release_handling() {
    let device_state = DeviceState::new();
    let mut previous_keys: Vec<Keycode> = Vec::new();
    let mut keys_sequence: Vec<String> = Vec::new();
    let mut sequence_start_time: u64 = 0;
    let mut is_sequence_active = false;
    
    loop {
        let keys = device_state.get_keys();
        let current_time = chrono::Utc::now().timestamp_millis() as u64;
        
        if previous_keys.is_empty() && !keys.is_empty() {
            is_sequence_active = true;
            sequence_start_time = current_time;
            
            for key in &keys {
                let key_str = format!("{:?}", key);
                if !keys_sequence.contains(&key_str) {
                    keys_sequence.push(key_str);
                }
            }
        } else if !keys.is_empty() && keys != previous_keys {
            for key in &keys {
                let key_str = format!("{:?}", key);
                if !keys_sequence.contains(&key_str) {
                    keys_sequence.push(key_str);
                }
            }
        } else if !previous_keys.is_empty() && keys.is_empty() && is_sequence_active {
            let end_time = current_time;
            let duration = end_time - sequence_start_time;
            
            let event = KeySequenceEvent {
                keys_pressed: keys_sequence.clone(),
                start_time: sequence_start_time,
                end_time,
                duration_ms: duration,
            };
            
            println!("{}", serde_json::to_string(&event).unwrap());
            
            keys_sequence.clear();
            is_sequence_active = false;
        }
        
        previous_keys = keys;
        thread::sleep(Duration::from_millis(10));
    }
}