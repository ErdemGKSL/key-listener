# Key Listener

A cross-platform key event listener and simulator for Windows and Linux.

## Features

- **Direct Handling**: Listens for physical key presses and outputs individual key events
- **Complex Handling**: Detects key combinations and outputs them as complex events
- **Hold and Release**: Tracks key press sequences with duration information
- **Key Simulation**: Receives JSON commands via standard input and simulates key events
- **Mouse Handling**: Tracks mouse movements and button events
- **Mouse Simulation**: Simulates mouse movements, clicks, and scroll events

## Requirements

- Rust and Cargo
- The following dependencies:
  - device_query
  - chrono
  - serde
  - serde_json
  - enigo

## Runtime dependencies for simulation (enigo)

Linux users may have to install `libxdo-dev` if they are using `X11`. For example, on Debian-based distros:

```Bash
apt install libxdo-dev
```

On Arch:

```Bash
pacman -S xdotool
```

On Fedora:

```Bash
dnf install libX11-devel libxdo-devel
```

On Gentoo:

```Bash
emerge -a xdotool
```

## Building

```bash
cargo build --release
```

## Usage

Run the program with one of the following modes:

```bash
# Direct key event handling (default)
./key-listener DIRECT

# Complex key combination handling
./key-listener COMPLEX

# Key hold and release tracking
./key-listener HOLD_AND_RELEASE

# Key simulation mode
./key-listener SIMULATION

# Mouse tracking mode
./key-listener MOUSE
```

## Key Simulation Mode

In simulation mode, the program accepts JSON input through standard input.

### Key Event JSON Format

```json
{
  "event_type": "key",
  "key": "a",
  "action": "tap",
  "delay_after_ms": 100
}
```

- `event_type`: Must be "key"
- `key`: The key to simulate (see supported keys below)
- `action`: One of "press", "release", or "tap"
- `delay_after_ms`: (Optional) Milliseconds to wait after the key action

### Mouse Event JSON Format

```json
{
  "event_type": "mouse",
  "action": "move",
  "x": 100,
  "y": 200,
  "button": "left",
  "scroll_x": 0,
  "scroll_y": -10,
  "delay_after_ms": 100
}
```

- `event_type`: Must be "mouse"
- `action`: One of "move", "click", "press", "release", or "scroll"
- `x`, `y`: X and Y coordinates for move actions
- `button`: One of "left", "right", "middle" for click/press/release actions
- `scroll_x`, `scroll_y`: Horizontal and vertical scroll amounts
- `delay_after_ms`: (Optional) Milliseconds to wait after the mouse action

### Supported Keys

- Single characters: "a", "b", "c", etc.
- Function keys: "F1", "F2", ..., "F20"
- Navigation keys: "Home", "End", "PageUp", "PageDown", "Delete", "Insert", "Escape", "Tab", "Return", "Space", "Backspace", "PrintScr"
- Arrow keys: "UpArrow", "DownArrow", "LeftArrow", "RightArrow"
- Modifier keys: "Alt", "Control", "Shift", "Meta", "Option", "CapsLock"
- Media keys: "VolumeUp", "VolumeDown", "VolumeMute", "MediaPlayPause", "MediaNextTrack", "MediaPrevTrack"
- On Windows: Numpad keys like "Numpad0", "Numpad1", etc. and "Num0", "Num1", etc.

### Examples

Tap the 'a' key:
```bash
echo '{"event_type":"key","key":"a","action":"tap","delay_after_ms":100}' | ./key-listener SIMULATION
```

Press and hold Shift, then tap 'a', then release Shift:
```bash
echo '{"event_type":"key","key":"Shift","action":"press"}' | ./key-listener SIMULATION
echo '{"event_type":"key","key":"a","action":"tap"}' | ./key-listener SIMULATION
echo '{"event_type":"key","key":"Shift","action":"release"}' | ./key-listener SIMULATION
```

Move mouse to position (100, 200):
```bash
echo '{"event_type":"mouse","action":"move","x":100,"y":200}' | ./key-listener SIMULATION
```

Click the left mouse button:
```bash
echo '{"event_type":"mouse","action":"click","button":"left"}' | ./key-listener SIMULATION
```

Scroll down 10 units:
```bash
echo '{"event_type":"mouse","action":"scroll","scroll_y":-10}' | ./key-listener SIMULATION
```

## Mouse Handling Mode

When in mouse handling mode, the program tracks mouse movements and button events and outputs them as JSON:

```json
{"event_type":"move","x":512,"y":384,"timestamp":1620000000000}
{"event_type":"button_press","x":512,"y":384,"button":"left","timestamp":1620000000100}
{"event_type":"button_release","x":512,"y":384,"button":"left","timestamp":1620000000200}
```

## License

MIT
