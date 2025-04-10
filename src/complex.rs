use std::{thread, time::Duration};
use device_query::{DeviceQuery, DeviceState, Keycode};
use crate::models::{ComplexKeyEvent, KeyEvent};

pub fn complex_handling() {
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
                        event_type: "direct".to_string(),
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
                        event_type: "direct".to_string(),
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
