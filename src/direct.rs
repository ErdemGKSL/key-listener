use std::{thread, time::Duration};
use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::models::KeyEvent;

pub fn direct_handling() {
    let device_state = DeviceState::new();
    let mut previous_keys: Vec<Keycode> = Vec::new();
    
    loop {
        let keys = device_state.get_keys();
        
        for key in &keys {
            if !previous_keys.contains(key) {
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
                let event = KeyEvent {
                    key: format!("{:?}", key),
                    pressed: false,
                    event_type: "direct".to_string(),
                    timestamp: chrono::Utc::now().timestamp_millis() as u64,
                };
                println!("{}", serde_json::to_string(&event).unwrap());
            }
        }
        
        previous_keys = keys;
        thread::sleep(Duration::from_millis(10));
    }
}