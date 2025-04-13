use std::{io::{self, BufRead}, thread, time::{Duration, Instant}};
use enigo::{Enigo, Keyboard, Settings};
use rdev::{simulate, Button, EventType, Key, SimulateError};

use crate::models::{KeySimulationEvent, MouseSimulationEvent, SimulationEvent, TextSimulationEvent};

// Helper function to send events and handle errors
fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            eprintln!("Error simulating event: {:?}", event_type);
        }
    }
    thread::sleep(Duration::from_millis(20));
}

// Map string names to rdev::Key variants
pub fn string_to_rdev_key(key_str: &str) -> Option<Key> {
    match key_str.to_lowercase().as_str() {
        "f1" => Some(Key::F1),
        "f2" => Some(Key::F2),
        "f3" => Some(Key::F3),
        "f4" => Some(Key::F4),
        "f5" => Some(Key::F5),
        "f6" => Some(Key::F6),
        "f7" => Some(Key::F7),
        "f8" => Some(Key::F8),
        "f9" => Some(Key::F9),
        "f10" => Some(Key::F10),
        "f11" => Some(Key::F11),
        "f12" => Some(Key::F12),

        "home" => Some(Key::Home),
        "end" => Some(Key::End),
        "pageup" => Some(Key::PageUp),
        "pagedown" => Some(Key::PageDown),
        "delete" => Some(Key::Delete),
        "insert" => Some(Key::Insert),
        "escape" | "esc" => Some(Key::Escape),
        "tab" => Some(Key::Tab),
        "return" | "enter" => Some(Key::Return),
        "space" => Some(Key::Space),
        "backspace" => Some(Key::Backspace),
        "printscreen" | "printscr" => Some(Key::PrintScreen),

        "uparrow" | "up" => Some(Key::UpArrow),
        "downarrow" | "down" => Some(Key::DownArrow),
        "leftarrow" | "left" => Some(Key::LeftArrow),
        "rightarrow" | "right" => Some(Key::RightArrow),

        "alt" => Some(Key::Alt),
        "control" | "ctrl" => Some(Key::ControlLeft),
        "shift" => Some(Key::ShiftLeft),
        "meta" | "win" | "cmd" | "command" => Some(Key::MetaLeft),
        "option" => Some(Key::Alt),
        "capslock" => Some(Key::CapsLock),

        "kp0" | "numpad0" => Some(Key::Kp0),
        "kp1" | "numpad1" => Some(Key::Kp1),
        "kp2" | "numpad2" => Some(Key::Kp2),
        "kp3" | "numpad3" => Some(Key::Kp3),
        "kp4" | "numpad4" => Some(Key::Kp4),
        "kp5" | "numpad5" => Some(Key::Kp5),
        "kp6" | "numpad6" => Some(Key::Kp6),
        "kp7" | "numpad7" => Some(Key::Kp7),
        "kp8" | "numpad8" => Some(Key::Kp8),
        "kp9" | "numpad9" => Some(Key::Kp9),
        "kpenter" | "kp_return" | "numpadenter" => Some(Key::KpReturn),
        "kp." | "kp_decimal" | "numpaddecimal" => Some(Key::KpDelete),
        "kp+" | "kp_plus" => Some(Key::KpPlus),
        "kp-" | "kp_minus" => Some(Key::KpMinus),
        "kp*" | "kp_multiply" => Some(Key::KpMultiply),
        "kp/" | "kp_divide" => Some(Key::KpDivide),

        "0" => Some(Key::Num0),
        "1" => Some(Key::Num1),
        "2" => Some(Key::Num2),
        "3" => Some(Key::Num3),
        "4" => Some(Key::Num4),
        "5" => Some(Key::Num5),
        "6" => Some(Key::Num6),
        "7" => Some(Key::Num7),
        "8" => Some(Key::Num8),
        "9" => Some(Key::Num9),

        "a" => Some(Key::KeyA),
        "b" => Some(Key::KeyB),
        "c" => Some(Key::KeyC),
        "d" => Some(Key::KeyD),
        "e" => Some(Key::KeyE),
        "f" => Some(Key::KeyF),
        "g" => Some(Key::KeyG),
        "h" => Some(Key::KeyH),
        "i" => Some(Key::KeyI),
        "j" => Some(Key::KeyJ),
        "k" => Some(Key::KeyK),
        "l" => Some(Key::KeyL),
        "m" => Some(Key::KeyM),
        "n" => Some(Key::KeyN),
        "o" => Some(Key::KeyO),
        "p" => Some(Key::KeyP),
        "q" => Some(Key::KeyQ),
        "r" => Some(Key::KeyR),
        "s" => Some(Key::KeyS),
        "t" => Some(Key::KeyT),
        "u" => Some(Key::KeyU),
        "v" => Some(Key::KeyV),
        "w" => Some(Key::KeyW),
        "x" => Some(Key::KeyX),
        "y" => Some(Key::KeyY),
        "z" => Some(Key::KeyZ),

        "`" => Some(Key::BackQuote),
        "-" => Some(Key::Minus),
        "=" => Some(Key::Equal),
        "[" => Some(Key::LeftBracket),
        "]" => Some(Key::RightBracket),
        "\\" => Some(Key::BackSlash),
        ";" => Some(Key::SemiColon),
        "'" => Some(Key::Quote),
        "," => Some(Key::Comma),
        "." => Some(Key::Dot),
        "/" => Some(Key::Slash),
        " " => Some(Key::Space),

        _ => None,
    }
}

// Map string names to rdev::Button variants
fn string_to_rdev_button(button_str: &str) -> Option<Button> {
    match button_str.to_lowercase().as_str() {
        "left" => Some(Button::Left),
        "right" => Some(Button::Right),
        "middle" => Some(Button::Middle),
        _ => None,
    }
}

pub fn key_simulation_handling() {
    let stdin = io::stdin();
    println!("Simulation mode active (using rdev). Listening for JSON input...");

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            match serde_json::from_str::<SimulationEvent>(&line) {
                Ok(simulation_event) => {
                    match simulation_event {
                        SimulationEvent::Key(key_event) => {
                            handle_key_event(key_event);
                        },
                        SimulationEvent::Mouse(mouse_event) => {
                            handle_mouse_event(mouse_event);
                        },
                        SimulationEvent::Text(text_event) => {
                            handle_text_event(text_event);
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error parsing JSON: {}. Expected format for \nkey: {{\"event_type\": \"key\", \"key\": \"a\", \"action\": \"tap\", \"delay_after_ms\": 100}}, \nmouse: {{\"event_type\": \"mouse\", \"action\": \"move\", \"x\": 100, \"y\": 200, \"duration_ms\": 500, \"ease\": \"linear\"}}, or \ntext: {{\"event_type\": \"text\", \"text\": \"hello\", \"delay_after_ms\": 50}}", e);
                }
            }
        }
    }
}

pub fn handle_key_event(event: KeySimulationEvent) {
    match string_to_rdev_key(&event.key) {
        Some(key) => {
            match event.action.as_str() {
                "press" => {
                    send(&EventType::KeyPress(key));
                }
                "release" => {
                    send(&EventType::KeyRelease(key));
                }
                "tap" | "click" => {
                    send(&EventType::KeyPress(key));
                    send(&EventType::KeyRelease(key));
                }
                _ => {
                    eprintln!("Unknown key action: {}. Valid actions are: press, release, tap/click",
                             event.action);
                }
            }

            if let Some(delay) = event.delay_after_ms {
                thread::sleep(Duration::from_millis(delay));
            }
        },
        None => eprintln!("Unsupported or unknown key: {}", event.key)
    }
}

// --- Easing Functions ---
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

pub fn handle_mouse_event(event: MouseSimulationEvent) {
    match event.action.as_str() {
        "move" => {
            if let (Some(target_x), Some(target_y)) = (event.x, event.y) {
                let target_x_f64 = target_x as f64;
                let target_y_f64 = target_y as f64;

                if let (Some(duration_ms), Some(ease_name)) = (event.duration_ms, &event.ease) {
                    if duration_ms > 0 {
                        eprintln!("Warning: Animated mouse movement with rdev requires knowing the start position. Simulating eased steps towards target, assuming start for curve calculation.");

                        let start_time = Instant::now();
                        let start_x = 0.0; // Assumed start for curve calculation
                        let start_y = 0.0; // Assumed start for curve calculation
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

                            send(&EventType::MouseMove { x: current_x, y: current_y });

                            thread::sleep(Duration::from_millis(10));
                        }
                        send(&EventType::MouseMove { x: target_x_f64, y: target_y_f64 });

                    } else {
                        send(&EventType::MouseMove { x: target_x_f64, y: target_y_f64 });
                    }
                } else {
                    send(&EventType::MouseMove { x: target_x_f64, y: target_y_f64 });
                }
            } else {
                eprintln!("Move action requires both x and y coordinates");
            }
        },
        "click" => {
            if let Some(button) = event.button.as_deref().and_then(string_to_rdev_button) {
                send(&EventType::ButtonPress(button));
                send(&EventType::ButtonRelease(button));
            } else {
                eprintln!("Invalid or unspecified button for click. Valid: left, right, middle");
            }
        },
        "press" => {
            if let Some(button) = event.button.as_deref().and_then(string_to_rdev_button) {
                send(&EventType::ButtonPress(button));
            } else {
                eprintln!("Invalid or unspecified button for press. Valid: left, right, middle");
            }
        },
        "release" => {
            if let Some(button) = event.button.as_deref().and_then(string_to_rdev_button) {
                send(&EventType::ButtonRelease(button));
            } else {
                eprintln!("Invalid or unspecified button for release. Valid: left, right, middle");
            }
        },
        "scroll" => {
            let target_scroll_x = event.scroll_x.unwrap_or(0) as i64;
            let target_scroll_y = event.scroll_y.unwrap_or(0) as i64;

            if target_scroll_x != 0 || target_scroll_y != 0 {
                if let (Some(duration_ms), Some(ease_name)) = (event.duration_ms, &event.ease) {
                    if duration_ms > 0 {
                        let start_time = Instant::now();
                        let start_scroll_x = 0.0;
                        let start_scroll_y = 0.0;
                        let target_scroll_x_f64 = target_scroll_x as f64;
                        let target_scroll_y_f64 = target_scroll_y as f64;
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

                        let mut last_scrolled_x = 0.0;
                        let mut last_scrolled_y = 0.0;

                        loop {
                            let elapsed = start_time.elapsed().as_millis() as f64;
                            if elapsed >= duration_f64 {
                                break;
                            }

                            let current_total_scroll_x = ease_func(elapsed, start_scroll_x, target_scroll_x_f64, duration_f64);
                            let current_total_scroll_y = ease_func(elapsed, start_scroll_y, target_scroll_y_f64, duration_f64);

                            let scroll_delta_x = current_total_scroll_x - last_scrolled_x;
                            let scroll_delta_y = current_total_scroll_y - last_scrolled_y;

                            let dx_to_send = scroll_delta_x.round() as i64;
                            let dy_to_send = scroll_delta_y.round() as i64;

                            if dx_to_send != 0 || dy_to_send != 0 {
                                send(&EventType::Wheel { delta_x: dx_to_send, delta_y: dy_to_send });
                                last_scrolled_x += dx_to_send as f64;
                                last_scrolled_y += dy_to_send as f64;
                            }

                            thread::sleep(Duration::from_millis(10));
                        }

                        let final_delta_x = target_scroll_x - last_scrolled_x.round() as i64;
                        let final_delta_y = target_scroll_y - last_scrolled_y.round() as i64;
                        if final_delta_x != 0 || final_delta_y != 0 {
                            send(&EventType::Wheel { delta_x: final_delta_x, delta_y: final_delta_y });
                        }

                    } else {
                        send(&EventType::Wheel { delta_x: target_scroll_x, delta_y: target_scroll_y });
                    }
                } else {
                    send(&EventType::Wheel { delta_x: target_scroll_x, delta_y: target_scroll_y });
                }
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

pub fn handle_text_event(event: TextSimulationEvent) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let _ = enigo.text(&event.text);
    if let Some(delay) = event.delay_after_ms {
        thread::sleep(Duration::from_millis(delay));
    }
}