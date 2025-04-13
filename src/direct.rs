use rdev::{listen, Event, EventType};
use crate::models::KeyEvent;
use chrono::Utc;

// Callback function to process key events
fn callback(event: Event) {
    let current_time = Utc::now().timestamp_millis() as u64;
    let key_event = match event.event_type {
        EventType::KeyPress(key) => Some(KeyEvent {
            key: format!("{:?}", key), // Use Debug representation of rdev::Key
            pressed: true,
            event_type: "direct".to_string(),
            timestamp: current_time,
        }),
        EventType::KeyRelease(key) => Some(KeyEvent {
            key: format!("{:?}", key), // Use Debug representation of rdev::Key
            pressed: false,
            event_type: "direct".to_string(),
            timestamp: current_time,
        }),
        // Ignore mouse events in this handler
        _ => None,
    };

    if let Some(ke) = key_event {
        if let Ok(json) = serde_json::to_string(&ke) {
            println!("{}", json);
        }
    }
}

pub fn direct_handling() {
    // This will block the thread and listen for events.
    if let Err(error) = listen(callback) {
        eprintln!("Error listening for keyboard events: {:?}", error);
    }
}