use std::{thread, time::Duration};
use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::models::KeySequenceEvent;

pub fn hold_and_release_handling() {
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

