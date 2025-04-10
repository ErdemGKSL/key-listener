use device_query::{DeviceQuery, DeviceState, Keycode};
use serde::{Serialize, Deserialize};
use std::thread;
use std::time::Duration;
use std::env;
use std::io::{self, BufRead};
use enigo::{Enigo, Key, Keyboard, Mouse, Settings};

#[derive(Serialize, Deserialize)]
struct KeyEvent {
    key: String,
    event_type: String,
    pressed: bool,
    timestamp: u64,
}

#[derive(Serialize, Deserialize)]
struct ComplexKeyEvent {
    keys: Vec<String>,
    event_type: String,
    timestamp: u64,
}

#[derive(Serialize, Deserialize)]
struct KeySequenceEvent {
    keys_pressed: Vec<String>,
    start_time: u64,
    end_time: u64,
    duration_ms: u64,
}

#[derive(Deserialize)]
struct KeySimulationEvent {
    key: String,
    action: String, // "press", "release", or "tap"
    delay_after_ms: Option<u64>,
}

#[derive(Deserialize)]
struct MouseSimulationEvent {
    action: String, // "move", "click", "press", "release", "scroll"
    x: Option<i32>,  // X coordinate for move actions
    y: Option<i32>,  // Y coordinate for move actions
    button: Option<String>, // "left", "right", "middle"
    scroll_x: Option<i32>, // Horizontal scroll amount
    scroll_y: Option<i32>, // Vertical scroll amount
    delay_after_ms: Option<u64>,
}

#[derive(Deserialize)]
#[serde(tag = "event_type")]
enum SimulationEvent {
    #[serde(rename = "key")]
    Key(KeySimulationEvent),
    #[serde(rename = "mouse")]
    Mouse(MouseSimulationEvent),
}

#[derive(Serialize, Deserialize)]
struct MouseEvent {
    event_type: String, // "move", "click", "scroll", etc.
    x: i32,
    y: i32,
    button: Option<String>, // "left", "right", "middle" for click events
    timestamp: u64,
}

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

fn string_to_key(key_str: &str) -> Option<Key> {
    match key_str {
        "F1" => Some(Key::F1),
        "F2" => Some(Key::F2),
        "F3" => Some(Key::F3),
        "F4" => Some(Key::F4),
        "F5" => Some(Key::F5),
        "F6" => Some(Key::F6),
        "F7" => Some(Key::F7),
        "F8" => Some(Key::F8),
        "F9" => Some(Key::F9),
        "F10" => Some(Key::F10),
        "F11" => Some(Key::F11),
        "F12" => Some(Key::F12),
        "F13" => Some(Key::F13),
        "F14" => Some(Key::F14),
        "F15" => Some(Key::F15),
        "F16" => Some(Key::F16),
        "F17" => Some(Key::F17),
        "F18" => Some(Key::F18),
        "F19" => Some(Key::F19),
        "F20" => Some(Key::F20),
        
        "Home" => Some(Key::Home),
        "End" => Some(Key::End),
        "PageUp" => Some(Key::PageUp),
        "PageDown" => Some(Key::PageDown),
        "Delete" => Some(Key::Delete),
        "Insert" => Some(Key::Insert),
        "Escape" => Some(Key::Escape),
        "Tab" => Some(Key::Tab),
        "Return" => Some(Key::Return),
        "Space" => Some(Key::Space),
        "Backspace" => Some(Key::Backspace),
        "PrintScr" => Some(Key::PrintScr),
        
        "UpArrow" => Some(Key::UpArrow),
        "DownArrow" => Some(Key::DownArrow),
        "LeftArrow" => Some(Key::LeftArrow),
        "RightArrow" => Some(Key::RightArrow),
        
        "Alt" => Some(Key::Alt),
        "Control" => Some(Key::Control),
        "Shift" => Some(Key::Shift),
        "Meta" => Some(Key::Meta),
        "Option" => Some(Key::Option),
        "CapsLock" => Some(Key::CapsLock),
        
        "VolumeUp" => Some(Key::VolumeUp),
        "VolumeDown" => Some(Key::VolumeDown),
        "VolumeMute" => Some(Key::VolumeMute),
        "MediaPlayPause" => Some(Key::MediaPlayPause),
        "MediaNextTrack" => Some(Key::MediaNextTrack),
        "MediaPrevTrack" => Some(Key::MediaPrevTrack),
        
        #[cfg(target_os = "windows")]
        "Numpad0" => Some(Key::Numpad0),
        #[cfg(target_os = "windows")]
        "Numpad1" => Some(Key::Numpad1),
        #[cfg(target_os = "windows")]
        "Numpad2" => Some(Key::Numpad2),
        #[cfg(target_os = "windows")]
        "Numpad3" => Some(Key::Numpad3),
        #[cfg(target_os = "windows")]
        "Numpad4" => Some(Key::Numpad4),
        #[cfg(target_os = "windows")]
        "Numpad5" => Some(Key::Numpad5),
        #[cfg(target_os = "windows")]
        "Numpad6" => Some(Key::Numpad6),
        #[cfg(target_os = "windows")]
        "Numpad7" => Some(Key::Numpad7),
        #[cfg(target_os = "windows")]
        "Numpad8" => Some(Key::Numpad8),
        #[cfg(target_os = "windows")]
        "Numpad9" => Some(Key::Numpad9),
        
        #[cfg(target_os = "windows")]
        "Num0" => Some(Key::Num0),
        #[cfg(target_os = "windows")]
        "Num1" => Some(Key::Num1),
        #[cfg(target_os = "windows")]
        "Num2" => Some(Key::Num2),
        #[cfg(target_os = "windows")]
        "Num3" => Some(Key::Num3),
        #[cfg(target_os = "windows")]
        "Num4" => Some(Key::Num4),
        #[cfg(target_os = "windows")]
        "Num5" => Some(Key::Num5),
        #[cfg(target_os = "windows")]
        "Num6" => Some(Key::Num6),
        #[cfg(target_os = "windows")]
        "Num7" => Some(Key::Num7),
        #[cfg(target_os = "windows")]
        "Num8" => Some(Key::Num8),
        #[cfg(target_os = "windows")]
        "Num9" => Some(Key::Num9),
        
        "Help" => Some(Key::Help),
        
        _ if key_str.len() == 1 => {
            let c = key_str.chars().next().unwrap();
            Some(Key::Unicode(c))
        },
        
        _ => None,
    }
}

fn key_simulation_handling() {
    let stdin = io::stdin();
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    println!("Simulation mode active. Listening for JSON input...");

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            match serde_json::from_str::<SimulationEvent>(&line) {
                Ok(simulation_event) => {
                    match simulation_event {
                        SimulationEvent::Key(key_event) => {
                            handle_key_event(&mut enigo, key_event);
                        },
                        SimulationEvent::Mouse(mouse_event) => {
                            handle_mouse_event(&mut enigo, mouse_event);
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error parsing JSON: {}. Expected format for key: {{\"event_type\": \"key\", \"key\": \"a\", \"action\": \"tap\", \"delay_after_ms\": 100}} or mouse: {{\"event_type\": \"mouse\", \"action\": \"move\", \"x\": 100, \"y\": 200}}", e);
                }
            }
        }
    }
}

fn handle_key_event(enigo: &mut Enigo, event: KeySimulationEvent) {
    match string_to_key(&event.key) {
        Some(key) => {
            match event.action.as_str() {
                "press" => {
                    let _ = enigo.key(key, enigo::Direction::Press);
                }
                "release" => {
                    let _ = enigo.key(key, enigo::Direction::Release);
                }
                "tap" => {
                    let _ = enigo.key(key, enigo::Direction::Click);
                }
                _ => {
                    eprintln!("Unknown key action: {}. Valid actions are: press, release, tap", 
                             event.action);
                }
            }

            if let Some(delay) = event.delay_after_ms {
                thread::sleep(Duration::from_millis(delay));
            }
        },
        None => eprintln!("Unsupported key: {}", event.key)
    }
}

fn handle_mouse_event(enigo: &mut Enigo, event: MouseSimulationEvent) {
    match event.action.as_str() {
        "move" => {
            if let (Some(x), Some(y)) = (event.x, event.y) {
                let _ = enigo.move_mouse(x, y, enigo::Coordinate::Abs);
            } else {
                eprintln!("Move action requires both x and y coordinates");
            }
        },
        "click" => {
            let button = match event.button.as_deref() {
                Some("left") => enigo::Button::Left,
                Some("right") => enigo::Button::Right,
                Some("middle") => enigo::Button::Middle,
                _ => {
                    eprintln!("Invalid button specified. Using left button as default.");
                    enigo::Button::Left
                }
            };
            let _ = enigo.button(button, enigo::Direction::Click);
        },
        "press" => {
            let button = match event.button.as_deref() {
                Some("left") => enigo::Button::Left,
                Some("right") => enigo::Button::Right,
                Some("middle") => enigo::Button::Middle,
                _ => {
                    eprintln!("Invalid button specified. Using left button as default.");
                    enigo::Button::Left
                }
            };
            let _ = enigo.button(button, enigo::Direction::Press);
        },
        "release" => {
            let button = match event.button.as_deref() {
                Some("left") => enigo::Button::Left,
                Some("right") => enigo::Button::Right,
                Some("middle") => enigo::Button::Middle,
                _ => {
                    eprintln!("Invalid button specified. Using left button as default.");
                    enigo::Button::Left
                }
            };
            let _ = enigo.button(button, enigo::Direction::Release);
        },
        "scroll" => {
            if let Some(x) = event.scroll_x {
                let _ = enigo.scroll(x, enigo::Axis::Horizontal);
            }
            
            if let Some(y) = event.scroll_y {
                let _ = enigo.scroll(y, enigo::Axis::Vertical);
            }
        },
        _ => {
            eprintln!("Unknown mouse action: {}. Valid actions are: move, click, press, release, scroll", 
                     event.action);
        }
    }
    
    if let Some(delay) = event.delay_after_ms {
        thread::sleep(Duration::from_millis(delay));
    }
}

fn mouse_handling() {
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