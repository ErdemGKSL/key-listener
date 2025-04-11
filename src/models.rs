use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct KeyEvent {
    pub key: String,
    pub event_type: String,
    pub pressed: bool,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ComplexKeyEvent {
    pub keys: Vec<String>,
    pub event_type: String,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize)]
pub struct KeySequenceEvent {
    pub keys: Vec<String>,
    pub start_time: u64,
    pub end_time: u64,
    pub duration_ms: u64,
    pub event_type: String
}

#[derive(Deserialize)]
pub struct KeySimulationEvent {
    pub key: String,
    pub action: String, // "press", "release", or "tap"
    pub delay_after_ms: Option<u64>,
}

#[derive(Deserialize)]
pub struct MouseSimulationEvent {
    pub action: String, // "move", "click", "press", "release", "scroll"
    pub x: Option<i32>,  // X coordinate for move actions
    pub y: Option<i32>,  // Y coordinate for move actions
    pub button: Option<String>, // "left", "right", "middle"
    pub scroll_x: Option<i32>, // Horizontal scroll amount
    pub scroll_y: Option<i32>, // Vertical scroll amount
    pub delay_after_ms: Option<u64>,
}

#[derive(Deserialize)]
#[serde(tag = "event_type")]
pub enum SimulationEvent {
    #[serde(rename = "key")]
    Key(KeySimulationEvent),
    #[serde(rename = "mouse")]
    Mouse(MouseSimulationEvent),
}

#[derive(Serialize, Deserialize)]
pub struct MouseEvent {
    pub event_type: String, // "move", "click", "scroll", etc.
    pub x: i32,
    pub y: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<String>, // "left", "right", "middle" for click events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressed: Option<bool>, // true for press, false for release
    pub timestamp: u64,
}