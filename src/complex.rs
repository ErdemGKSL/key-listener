use rdev::{listen, Event, EventType, Key};
use crate::models::{ComplexKeyEvent, KeyEvent};
use chrono::Utc;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub fn complex_handling() {
    // Shared state for pressed keys and the current combination string
    let pressed_keys = Arc::new(Mutex::new(HashSet::<Key>::new()));
    let current_combination = Arc::new(Mutex::new(Vec::<String>::new()));

    // Clone Arcs for the callback closure
    let pressed_keys_clone = Arc::clone(&pressed_keys);
    let current_combination_clone = Arc::clone(&current_combination);

    let callback = move |event: Event| {
        let mut pressed_keys = pressed_keys_clone.lock().unwrap();
        let mut current_combination = current_combination_clone.lock().unwrap();
        let current_time = Utc::now().timestamp_millis() as u64;
        let was_empty = pressed_keys.is_empty();

        match event.event_type {
            EventType::KeyPress(key) => {
                let key_str = format!("{:?}", key);
                let is_new_press = pressed_keys.insert(key);

                if is_new_press {
                    // Add to string combination if not already present
                    if !current_combination.contains(&key_str) {
                         current_combination.push(key_str.clone());
                    }

                    // Output direct key press event
                    let key_press_event = KeyEvent {
                        key: key_str.clone(),
                        pressed: true,
                        event_type: "direct".to_string(),
                        timestamp: current_time,
                    };
                    if let Ok(json) = serde_json::to_string(&key_press_event) {
                        println!("{}", json);
                    }

                    // Output complex event
                    let complex_event = ComplexKeyEvent {
                        keys: current_combination.clone(),
                        event_type: if was_empty { "press".to_string() } else { "combination".to_string() },
                        timestamp: current_time,
                    };
                    if let Ok(json) = serde_json::to_string(&complex_event) {
                        println!("{}", json);
                    }
                }
            }
            EventType::KeyRelease(key) => {
                let key_str = format!("{:?}", key);
                if pressed_keys.remove(&key) {
                     // Remove from string combination
                    if let Some(index) = current_combination.iter().position(|k| k == &key_str) {
                        current_combination.remove(index);
                    }

                    // Output direct key release event
                    let key_release_event = KeyEvent {
                        key: key_str,
                        pressed: false,
                        event_type: "direct".to_string(),
                        timestamp: current_time,
                    };
                     if let Ok(json) = serde_json::to_string(&key_release_event) {
                        println!("{}", json);
                    }

                    // Output complex event only if it's the last key being released
                    if pressed_keys.is_empty() && !current_combination.is_empty() {
                         let complex_event = ComplexKeyEvent {
                            keys: current_combination.clone(), // Should be empty now conceptually, but send last state
                            event_type: "release".to_string(),
                            timestamp: current_time,
                        };
                        if let Ok(json) = serde_json::to_string(&complex_event) {
                            println!("{}", json);
                        }
                        // Clear combination string only after the final release event is sent
                        current_combination.clear();
                    } else if !pressed_keys.is_empty() {
                         // If other keys are still pressed, send a combination update
                         let complex_event = ComplexKeyEvent {
                            keys: current_combination.clone(),
                            event_type: "combination".to_string(),
                            timestamp: current_time,
                        };
                        if let Ok(json) = serde_json::to_string(&complex_event) {
                            println!("{}", json);
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
