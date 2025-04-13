use rdev::{listen, Event, EventType, Key};
use crate::models::KeySequenceEvent;
use chrono::Utc;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub fn hold_and_release_handling() {
    // Shared state
    let pressed_keys = Arc::new(Mutex::new(HashSet::<Key>::new()));
    let keys_sequence = Arc::new(Mutex::new(Vec::<String>::new()));
    let sequence_start_time = Arc::new(Mutex::new(None::<u64>));

    // Clone Arcs for the callback
    let pressed_keys_clone = Arc::clone(&pressed_keys);
    let keys_sequence_clone = Arc::clone(&keys_sequence);
    let sequence_start_time_clone = Arc::clone(&sequence_start_time);

    let callback = move |event: Event| {
        let mut pressed_keys = pressed_keys_clone.lock().unwrap();
        let mut keys_sequence = keys_sequence_clone.lock().unwrap();
        let mut start_time = sequence_start_time_clone.lock().unwrap();
        let current_time = Utc::now().timestamp_millis() as u64;

        match event.event_type {
            EventType::KeyPress(key) => {
                let key_str = format!("{:?}", key);
                let is_new_press = pressed_keys.insert(key);

                if is_new_press {
                    // Start sequence on first key press
                    if start_time.is_none() {
                        *start_time = Some(current_time);
                    }
                    // Add key to sequence if not already present
                    if !keys_sequence.contains(&key_str) {
                        keys_sequence.push(key_str);
                    }
                }
            }
            EventType::KeyRelease(key) => {
                if pressed_keys.remove(&key) {
                    // Check if this was the last pressed key
                    if pressed_keys.is_empty() {
                        if let Some(st) = *start_time {
                            let end_time = current_time;
                            let duration = end_time - st;

                            let sequence_event = KeySequenceEvent {
                                event_type: "key_sequence".to_string(),
                                keys: keys_sequence.clone(),
                                start_time: st,
                                end_time,
                                duration_ms: duration,
                            };

                            if let Ok(json) = serde_json::to_string(&sequence_event) {
                                println!("{}", json);
                            }

                            // Reset sequence state
                            keys_sequence.clear();
                            *start_time = None;
                        }
                    }
                }
            }
            _ => {} // Ignore other event types
        }
    };

    if let Err(error) = listen(callback) {
        eprintln!("Error listening for keyboard events: {:?}", error);
    }
}

