use rdev::{listen, Event, EventType, Button};
use crate::models::MouseEvent;
use chrono::Utc;

// Helper function to map rdev Button to string
fn button_to_string(button: Button) -> String {
    match button {
        Button::Left => "left".to_string(),
        Button::Right => "right".to_string(),
        Button::Middle => "middle".to_string(),
        Button::Unknown(code) => format!("button_{}", code), // Handle unknown buttons
    }
}

// Callback function to process events
fn callback(event: Event) {
    let current_time = Utc::now().timestamp_millis() as u64;
    let mouse_event = match event.event_type {
        EventType::MouseMove { x, y } => Some(MouseEvent {
            event_type: "move".to_string(),
            x: Some(x as i32), // Wrap in Some
            y: Some(y as i32), // Wrap in Some
            button: None,
            pressed: None,
            delta_x: None,
            delta_y: None,
            timestamp: current_time,
        }),
        EventType::ButtonPress(button) => Some(MouseEvent {
            event_type: "button".to_string(),
            // Coordinates are not directly available for button events in rdev
            x: None, // Set to None
            y: None, // Set to None
            button: Some(button_to_string(button)),
            pressed: Some(true),
            delta_x: None,
            delta_y: None,
            timestamp: current_time,
        }),
        EventType::ButtonRelease(button) => Some(MouseEvent {
            event_type: "button".to_string(),
            // Coordinates are not directly available for button events in rdev
            x: None, // Set to None
            y: None, // Set to None
            button: Some(button_to_string(button)),
            pressed: Some(false),
            delta_x: None,
            delta_y: None,
            timestamp: current_time,
        }),
        EventType::Wheel { delta_x, delta_y } => Some(MouseEvent {
            event_type: "scroll".to_string(),
            // Coordinates are not directly available for wheel events in rdev
            x: None, // Set to None
            y: None, // Set to None
            button: None,
            pressed: None,
            delta_x: Some(delta_x as i32), // Cast i64 to i32
            delta_y: Some(delta_y as i32), // Cast i64 to i32
            timestamp: current_time,
        }),
        // Ignore keyboard events in this handler
        EventType::KeyPress(_) | EventType::KeyRelease(_) => None,
    };

    if let Some(me) = mouse_event {
        if let Ok(json) = serde_json::to_string(&me) {
            println!("{}", json);
        }
    }
}

pub fn mouse_handling() {
    if let Err(error) = listen(callback) {
        eprintln!("Error listening for mouse events: {:?}", error);
    }
}