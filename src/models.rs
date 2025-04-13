use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct KeyEvent {
    pub key: String,
    pub event_type: String,
    pub pressed: bool,
    pub timestamp: u64,
}

#[derive(Serialize)]
pub struct ComplexKeyEvent {
    pub keys: Vec<String>,
    pub event_type: String,
    pub timestamp: u64,
}

#[derive(Serialize)]
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
    pub duration_ms: Option<u64>, // Duration for animated movement
    pub ease: Option<String>, // Easing function name ("linear", "easeInQuad", etc.)
}

#[derive(Deserialize)]
pub struct TextSimulationEvent {
    pub text: String,
    pub delay_after_ms: Option<u64>,
}

#[derive(Deserialize)]
#[serde(tag = "event_type")]
pub enum SimulationEvent {
    #[serde(rename = "key")]
    Key(KeySimulationEvent),
    #[serde(rename = "mouse")]
    Mouse(MouseSimulationEvent),
    #[serde(rename = "text")]
    Text(TextSimulationEvent),
}

#[derive(Serialize)]
pub struct MouseEvent {
    pub event_type: String, // "move", "click", "scroll", etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<String>, // "left", "right", "middle" for click events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressed: Option<bool>, // true for press, false for release
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_x: Option<i32>, // Horizontal scroll delta
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_y: Option<i32>, // Vertical scroll delta
    pub timestamp: u64,
}