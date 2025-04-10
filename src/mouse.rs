use std::{thread, time::Duration};
use device_query::{DeviceState, DeviceQuery};

use crate::models::MouseEvent;

pub fn mouse_handling() {
    let device_state = DeviceState::new();
    let mut previous_position = (0, 0);
    let mut previous_buttons = Vec::new();
    
    loop {
        let mouse = device_state.get_mouse();
        let buttons = mouse.button_pressed;
        let position = mouse.coords;
        let current_time = chrono::Utc::now().timestamp_millis() as u64;
        
        if position != previous_position {
            let event = MouseEvent {
                event_type: "move".to_string(),
                x: position.0,
                y: position.1,
                button: None,
                timestamp: current_time,
            };
            
            println!("{}", serde_json::to_string(&event).unwrap());
            previous_position = position;
        }
        
        if buttons.len() != previous_buttons.len() || buttons.iter().zip(previous_buttons.iter()).any(|(a, b)| a != b) {
            for (index, &pressed) in buttons.iter().enumerate() {
                let previous_pressed = previous_buttons.get(index).unwrap_or(&false);
                
                if pressed && !previous_pressed {
                    let button_name = match index {
                        0 => "left".to_string(),
                        1 => "right".to_string(),
                        2 => "middle".to_string(),
                        _ => format!("button_{}", index),
                    };
                    
                    let event = MouseEvent {
                        event_type: "button_press".to_string(),
                        x: position.0,
                        y: position.1,
                        button: Some(button_name),
                        timestamp: current_time,
                    };
                    println!("{}", serde_json::to_string(&event).unwrap());
                } else if !pressed && *previous_pressed {
                    let button_name = match index {
                        0 => "left".to_string(),
                        1 => "right".to_string(),
                        2 => "middle".to_string(),
                        _ => format!("button_{}", index),
                    };
                    
                    let event = MouseEvent {
                        event_type: "button_release".to_string(),
                        x: position.0,
                        y: position.1,
                        button: Some(button_name),
                        timestamp: current_time,
                    };
                    println!("{}", serde_json::to_string(&event).unwrap());
                }
            }
            
            previous_buttons = buttons.clone();
        }
        
        thread::sleep(Duration::from_millis(10));
    }
}