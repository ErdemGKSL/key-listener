use std::{io::{self, BufRead}, thread, time::{Duration, Instant}};
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

// --- Easing Functions ---
// t: current time, b: beginning value, c: change in value, d: duration
fn linear_tween(t: f64, b: f64, c: f64, d: f64) -> f64 {
    c * t / d + b
}

fn ease_in_quad(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t = t / d;
    c * t * t + b
}

fn ease_out_quad(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t = t / d;
    -c * t * (t - 2.0) + b
}

fn ease_in_out_quad(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let mut t = t / (d / 2.0);
    if t < 1.0 {
        return c / 2.0 * t * t + b;
    }
    t -= 1.0;
    -c / 2.0 * (t * (t - 2.0) - 1.0) + b
}

fn ease_in_cubic(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t = t / d;
    c * t * t * t + b
}

fn ease_out_cubic(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t = t / d - 1.0;
    c * (t * t * t + 1.0) + b
}

fn ease_in_out_cubic(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let mut t = t / (d / 2.0);
    if t < 1.0 {
        return c / 2.0 * t * t * t + b;
    }
    t -= 2.0;
    c / 2.0 * (t * t * t + 2.0) + b
}

fn ease_in_sine(t: f64, b: f64, c: f64, d: f64) -> f64 {
    -c * (t / d * std::f64::consts::PI / 2.0).cos() + c + b
}

fn ease_out_sine(t: f64, b: f64, c: f64, d: f64) -> f64 {
    c * (t / d * std::f64::consts::PI / 2.0).sin() + b
}

fn ease_in_out_sine(t: f64, b: f64, c: f64, d: f64) -> f64 {
    -c / 2.0 * ((std::f64::consts::PI * t / d).cos() - 1.0) + b
}
// --- End Easing Functions ---

pub fn handle_mouse_event(enigo: &mut Enigo, event: MouseSimulationEvent) {
    match event.action.as_str() {
        "move" => {
            if let (Some(target_x), Some(target_y)) = (event.x, event.y) {
                if let (Some(duration_ms), Some(ease_name)) = (event.duration_ms, &event.ease) {
                    if duration_ms > 0 {
                        // Animated move
                        let start_time = Instant::now();
                        let start_pos = enigo.location().unwrap_or((0, 0));
                        let start_x = start_pos.0 as f64;
                        let start_y = start_pos.1 as f64;
                        let target_x_f64 = target_x as f64;
                        let target_y_f64 = target_y as f64;
                        let change_x = target_x_f64 - start_x;
                        let change_y = target_y_f64 - start_y;
                        let duration_f64 = duration_ms as f64;

                        let ease_func: fn(f64, f64, f64, f64) -> f64 = match ease_name.as_str() {
                            "linear" => linear_tween,
                            "easeInQuad" => ease_in_quad,
                            "easeOutQuad" => ease_out_quad,
                            "easeInOutQuad" => ease_in_out_quad,
                            "easeInCubic" => ease_in_cubic,
                            "easeOutCubic" => ease_out_cubic,
                            "easeInOutCubic" => ease_in_out_cubic,
                            "easeInSine" => ease_in_sine,
                            "easeOutSine" => ease_out_sine,
                            "easeInOutSine" => ease_in_out_sine,
                            _ => {
                                eprintln!("Unknown ease function: '{}'. Defaulting to linear.", ease_name);
                                linear_tween
                            }
                        };

                        loop {
                            let elapsed = start_time.elapsed().as_millis() as f64;
                            if elapsed >= duration_f64 {
                                break;
                            }

                            let current_x = ease_func(elapsed, start_x, change_x, duration_f64);
                            let current_y = ease_func(elapsed, start_y, change_y, duration_f64);

                            let _ = enigo.move_mouse(current_x as i32, current_y as i32, enigo::Coordinate::Abs);
                            // Small sleep to yield control and manage update rate
                            thread::sleep(Duration::from_millis(5));
                        }
                        // Ensure final position is exact
                        let _ = enigo.move_mouse(target_x, target_y, enigo::Coordinate::Abs);

                    } else {
                        // Instant move if duration is 0
                        let _ = enigo.move_mouse(target_x, target_y, enigo::Coordinate::Abs);
                    }
                } else {
                    // Instant move if duration or ease is not specified
                    let _ = enigo.move_mouse(target_x, target_y, enigo::Coordinate::Abs);
                }
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
                    eprintln!("Invalid button specified for click. Using left button as default.");
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
                    eprintln!("Invalid button specified for press. Using left button as default.");
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
                    eprintln!("Invalid button specified for release. Using left button as default.");
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