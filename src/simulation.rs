use std::{io::{self, BufRead}, thread, time::Duration};
use enigo::{Enigo, Key, Keyboard, Mouse, Settings};

use crate::models::{KeySimulationEvent, MouseSimulationEvent, SimulationEvent};

pub fn string_to_key(key_str: &str) -> Option<Key> {
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

pub fn key_simulation_handling() {
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

pub fn handle_key_event(enigo: &mut Enigo, event: KeySimulationEvent) {
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

pub fn handle_mouse_event(enigo: &mut Enigo, event: MouseSimulationEvent) {
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