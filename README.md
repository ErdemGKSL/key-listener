# Key Listener

[![Build and Release](https://github.com/yourusername/key-listener/actions/workflows/build-and-release.yml/badge.svg)](https://github.com/yourusername/key-listener/actions/workflows/build-and-release.yml)
[![CI](https://github.com/yourusername/key-listener/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/key-listener/actions/workflows/ci.yml)
[![Nightly Build](https://github.com/yourusername/key-listener/actions/workflows/nightly.yml/badge.svg)](https://github.com/yourusername/key-listener/actions/workflows/nightly.yml)

A cross-platform key event listener and simulator for Windows, Linux, and macOS.

## Features

- **Direct Handling**: Listens for physical key presses and outputs individual key events
- **Complex Handling**: Detects key combinations and outputs them as complex events
- **Hold and Release**: Tracks key press sequences with duration information
- **Key Simulation**: Receives JSON commands via standard input and simulates key events
- **Mouse Handling**: Tracks mouse movements and button events
- **Mouse Simulation**: Simulates mouse movements (instant or animated), clicks, and scroll events (instant or animated)
- **Text Simulation**: Simulates typing a string of text.

## Downloads

### Stable Releases
Download the latest stable release from the [Releases page](https://github.com/yourusername/key-listener/releases/latest).

### Development Builds
Automatic development builds are available for the latest code:
- **Dev releases**: Created on every push to `main` or `develop` branches
- **Nightly builds**: Optimized builds created daily at 2 AM UTC

### Available Platforms
- **Linux**: x86_64, ARM64, i686
- **Windows**: x86_64, i686
- **macOS**: x86_64

## Requirements

- Rust and Cargo (for building from source)
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

### Quick Build (All Features)

To build the main binary with all features enabled:

```bash
cargo build --release
```

This creates a single binary `target/release/key-listener` that includes all functionality.

### Feature-Specific Builds

You can build smaller, specialized binaries that only include specific features:

#### Using the Build Script (Recommended)

```bash
./build.sh
```

This builds all variants automatically:
- `key-listener` - All features enabled
- `key-listener-direct` - Direct key event listening only
- `key-listener-complex` - Complex key combination handling only  
- `key-listener-hold-release` - Hold and release sequence tracking only
- `key-listener-simulation` - Key/mouse simulation only
- `key-listener-mouse` - Mouse event listening only

#### Manual Feature Builds

```bash
# Direct mode only (smallest binary for simple key listening)
cargo build --release --bin key-listener-direct --features direct --no-default-features

# Complex mode only (for key combinations)
cargo build --release --bin key-listener-complex --features complex --no-default-features

# Hold and release mode only (for sequence tracking with timing)
cargo build --release --bin key-listener-hold-release --features hold_and_release --no-default-features

# Simulation mode only (for automating key/mouse input)
cargo build --release --bin key-listener-simulation --features simulation --no-default-features

# Mouse mode only (for mouse event tracking)
cargo build --release --bin key-listener-mouse --features mouse --no-default-features
```

### Build Advantages

- **Smaller binaries**: Feature-specific builds are significantly smaller
- **Reduced dependencies**: Only includes libraries needed for specific features
- **Better performance**: Less code means faster startup and lower memory usage
- **Security**: Reduced attack surface with fewer included features

### Binary Sizes (Approximate)

- `key-listener` (all features): ~920KB
- `key-listener-simulation`: ~813KB (includes enigo for simulation)
- `key-listener-hold-release`: ~514KB
- `key-listener-complex`: ~518KB
- `key-listener-direct`: ~490KB (smallest)
- `key-listener-mouse`: ~486KB

### Using Makefile (Alternative)

For convenience, you can also use the included Makefile:

```bash
# Build all variants
make all
# or simply
make

# Build individual variants
make build-direct
make build-complex
make build-hold-release
make build-simulation
make build-mouse

# Build main binary with all features
make build

# Clean build artifacts
make clean

# Show available targets
make help
```

### Cross-Compilation

For building Windows binaries on Linux or other cross-compilation needs:

```bash
# Build for all supported targets
./cross-build.sh

# Build for specific target
./cross-build.sh x86_64-pc-windows-gnu
```

## CI/CD Pipeline

This project uses GitHub Actions for automated building, testing, and releasing:

### Workflows

1. **CI** (`ci.yml`): Runs on pull requests and feature branches
   - Code formatting check (`cargo fmt`)
   - Linting with Clippy (`cargo clippy`)
   - Unit tests on multiple platforms
   - Feature-specific build tests
   - Binary size analysis

2. **Build and Release** (`build-and-release.yml`): Runs on main/develop branches
   - Cross-platform builds (Linux, Windows, macOS, ARM64)
   - Automatic dev release creation
   - Cleanup of old dev releases (keeps latest 5)
   - Artifact uploads for download

3. **Stable Release** (`stable-release.yml`): Runs on tagged releases
   - Production builds with all platforms
   - Compressed archives (tar.gz, zip)
   - Stable release attachments

4. **Nightly Build** (`nightly.yml`): Runs daily at 2 AM UTC
   - Optimized builds with LTO and size optimization
   - Stripped binaries for smaller size
   - Performance-focused compilation flags

### Release Types

- **Stable releases**: Tagged releases (e.g., `v1.0.0`)
- **Dev releases**: Automatic builds from main/develop (e.g., `dev-20241202-123456`)
- **Nightly releases**: Daily optimized builds (tag: `nightly`)
- **PR artifacts**: Build artifacts for pull requests (3-day retention)

### Supported Platforms

All workflows build for these platforms:
- `x86_64-unknown-linux-gnu` (Linux 64-bit)
- `x86_64-pc-windows-msvc` (Windows 64-bit)
- `x86_64-apple-darwin` (macOS 64-bit)
- `aarch64-unknown-linux-gnu` (Linux ARM64)
- `i686-unknown-linux-gnu` (Linux 32-bit, stable only)
- `i686-pc-windows-msvc` (Windows 32-bit, stable only)

## Usage

### Running the Binaries

#### Full-Featured Binary (All Modes)
```bash
# Run with all features (default: direct mode)
./target/release/key-listener

# Specify mode explicitly
./target/release/key-listener direct
./target/release/key-listener complex
./target/release/key-listener hold_and_release
./target/release/key-listener simulation
./target/release/key-listener mouse
```

#### Feature-Specific Binaries
```bash
# Direct mode (simple key events)
./target/release/key-listener-direct

# Complex mode (key combinations)
./target/release/key-listener-complex

# Hold and release mode (timing sequences)
./target/release/key-listener-hold-release

# Simulation mode (automation)
./target/release/key-listener-simulation

# Mouse mode (mouse events)
./target/release/key-listener-mouse
```

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

In simulation mode, the program accepts JSON objects through standard input, one per line, to simulate keyboard, mouse, and text actions. Each JSON object must specify an `event_type` ("key", "mouse", or "text") and an `action` (for key/mouse).

### Key Simulation (`event_type: "key"`)

Used to simulate keyboard key presses, releases, or taps.

**JSON Fields:**

*   `event_type`: (Required) Must be `"key"`.
*   `key`: (Required) A string representing the key to simulate. See [Supported Keys](#supported-keys).
*   `action`: (Required) The action to perform on the key. Must be one of:
    *   `"press"`: Simulates pressing and holding down the specified key. The key remains pressed until a corresponding `"release"` action is sent.
    *   `"release"`: Simulates releasing a previously pressed key.
    *   `"tap"`: Simulates a quick press and release of the key (a click).
*   `delay_after_ms`: (Optional) An integer specifying the number of milliseconds to pause *after* executing the key action. Defaults to 0 if omitted.

**Examples:**

*   **Tap the 'a' key and wait 100ms:**
    ```json
    {"event_type":"key","key":"a","action":"tap","delay_after_ms":100}
    ```
    ```bash
    echo '{"event_type":"key","key":"a","action":"tap","delay_after_ms":100}' | ./key-listener SIMULATION
    ```
*   **Type 'Hello': Press Shift, tap 'h', release Shift, tap 'e', 'l', 'l', 'o':**
    ```json
    {"event_type":"key","key":"Shift","action":"press"}
    {"event_type":"key","key":"h","action":"tap"}
    {"event_type":"key","key":"Shift","action":"release"}
    {"event_type":"key","key":"e","action":"tap"}
    {"event_type":"key","key":"l","action":"tap"}
    {"event_type":"key","key":"l","action":"tap"}
    {"event_type":"key","key":"o","action":"tap"}
    ```
    ```bash
    # Send each line separately or pipe a file with one JSON per line
    echo '{"event_type":"key","key":"Shift","action":"press"}' | ./key-listener SIMULATION
    echo '{"event_type":"key","key":"h","action":"tap"}' | ./key-listener SIMULATION
    echo '{"event_type":"key","key":"Shift","action":"release"}' | ./key-listener SIMULATION
    echo '{"event_type":"key","key":"e","action":"tap"}' | ./key-listener SIMULATION
    echo '{"event_type":"key","key":"l","action":"tap"}' | ./key-listener SIMULATION
    echo '{"event_type":"key","key":"l","action":"tap"}' | ./key-listener SIMULATION
    echo '{"event_type":"key","key":"o","action":"tap"}' | ./key-listener SIMULATION
    ```

### Mouse Simulation (`event_type: "mouse"`)

Used to simulate mouse movements, button clicks/presses/releases, and scrolling.

**Common JSON Fields:**

*   `event_type`: (Required) Must be `"mouse"`.
*   `action`: (Required) The mouse action to perform. See specific actions below.
*   `delay_after_ms`: (Optional) An integer specifying the number of milliseconds to pause *after* executing the mouse action. Defaults to 0 if omitted.

**Actions:**

1.  **`action: "move"`**
    *   Simulates moving the mouse cursor to a specific screen coordinate.
    *   **Required Fields:**
        *   `x`: Target X coordinate (absolute pixel value).
        *   `y`: Target Y coordinate (absolute pixel value).
    *   **Optional Fields for Animation:**
        *   `duration_ms`: Duration in milliseconds for the movement animation. If 0 or omitted, the move is instantaneous.
        *   `ease`: The name of the easing function for the animation (e.g., `"easeInOutQuad"`). Requires `duration_ms` > 0. See [Supported Easing Functions](#supported-easing-functions-for-mouse-actions). Defaults to `"linear"` if `duration_ms` is provided but `ease` is omitted.
    *   **Examples:**
        *   **Instant Move:** Move cursor to (100, 200).
          ```json
          {"event_type":"mouse","action":"move","x":100,"y":200}
          ```
          ```bash
          echo '{"event_type":"mouse","action":"move","x":100,"y":200}' | ./key-listener SIMULATION
          ```
        *   **Animated Move:** Move cursor smoothly to (500, 500) over 1 second using `easeInOutQuad` easing.
          ```json
          {"event_type":"mouse","action":"move","x":500,"y":500,"duration_ms":1000,"ease":"easeInOutQuad"}
          ```
          ```bash
          echo '{"event_type":"mouse","action":"move","x":500,"y":500,"duration_ms":1000,"ease":"easeInOutQuad"}' | ./key-listener SIMULATION
          ```

2.  **`action: "click"`**
    *   Simulates a full click (press and release) of a mouse button.
    *   **Optional Fields:**
        *   `button`: The button to click. Can be `"left"`, `"right"`, or `"middle"`. Defaults to `"left"` if omitted.
    *   **Example:**
        *   **Right Click:**
          ```json
          {"event_type":"mouse","action":"click","button":"right"}
          ```
          ```bash
          echo '{"event_type":"mouse","action":"click","button":"right"}' | ./key-listener SIMULATION
          ```

3.  **`action: "press"`**
    *   Simulates pressing and holding down a mouse button. The button remains pressed until a corresponding `"release"` action is sent.
    *   **Optional Fields:**
        *   `button`: The button to press. Can be `"left"`, `"right"`, or `"middle"`. Defaults to `"left"` if omitted.
    *   **Example:**
        *   **Press Left Button:**
          ```json
          {"event_type":"mouse","action":"press","button":"left"}
          ```
          ```bash
          echo '{"event_type":"mouse","action":"press","button":"left"}' | ./key-listener SIMULATION
          ```

4.  **`action: "release"`**
    *   Simulates releasing a previously pressed mouse button.
    *   **Optional Fields:**
        *   `button`: The button to release. Can be `"left"`, `"right"`, or `"middle"`. Defaults to `"left"` if omitted.
    *   **Example:**
        *   **Release Left Button:**
          ```json
          {"event_type":"mouse","action":"release","button":"left"}
          ```
          ```bash
          echo '{"event_type":"mouse","action":"release","button":"left"}' | ./key-listener SIMULATION
          ```

5.  **`action: "scroll"`**
    *   Simulates scrolling the mouse wheel. Can be instant or animated.
    *   **Optional Fields:** (At least one of `delta_x` or `delta_y` must be provided)
        *   `delta_x`: The total amount to scroll horizontally. Positive values scroll right, negative values scroll left.
        *   `delta_y`: The total amount to scroll vertically. Positive values scroll up, negative values scroll down. (Note: Vertical scroll direction might feel inverted depending on OS settings).
    *   **Optional Fields for Animation:**
        *   `duration_ms`: Duration in milliseconds for the scroll animation. If 0 or omitted, the scroll is instantaneous.
        *   `ease`: The name of the easing function for the animation (e.g., `"easeOutSine"`). Requires `duration_ms` > 0. See [Supported Easing Functions](#supported-easing-functions-for-mouse-actions). Defaults to `"linear"` if `duration_ms` is provided but `ease` is omitted.
    *   **Examples:**
        *   **Instant Scroll Down 10 units:**
          ```json
          {"event_type":"mouse","action":"scroll","delta_y":-10}
          ```
          ```bash
          echo '{"event_type":"mouse","action":"scroll","delta_y":-10}' | ./key-listener SIMULATION
          ```
        *   **Animated Scroll Right 50 units over 500ms:**
          ```json
          {"event_type":"mouse","action":"scroll","delta_x":50,"duration_ms":500,"ease":"easeOutSine"}
          ```
          ```bash
          echo '{"event_type":"mouse","action":"scroll","delta_x":50,"duration_ms":500,"ease":"easeOutSine"}' | ./key-listener SIMULATION
          ```

### Text Simulation (`event_type: "text"`)

Used to simulate typing a string of text directly. This is often simpler than sending individual key tap events for each character.

**JSON Fields:**

*   `event_type`: (Required) Must be `"text"`.
*   `text`: (Required) The string of text to type.
*   `delay_after_ms`: (Optional) An integer specifying the number of milliseconds to pause *after* typing the text. Defaults to 0 if omitted.

**Example:**

*   **Type "Hello, World!" and wait 200ms:**
    ```json
    {"event_type":"text","text":"Hello, World!","delay_after_ms":200}
    ```
    ```bash
    echo '{"event_type":"text","text":"Hello, World!","delay_after_ms":200}' | ./key-listener SIMULATION
    ```

### Supported Easing Functions for Mouse Actions

These easing functions can be used with `action: "move"` and `action: "scroll"` when `duration_ms` is provided.

- `linear`
- `easeInQuad`
- `easeOutQuad`
- `easeInOutQuad`
- `easeInCubic`
- `easeOutCubic`
- `easeInOutCubic`
- `easeInSine`
- `easeOutSine`
- `easeInOutSine`

### Supported Keys

- Single characters: "a", "b", "c", etc.
- Function keys: "F1", "F2", ..., "F20"
- Navigation keys: "Home", "End", "PageUp", "PageDown", "Delete", "Insert", "Escape", "Tab", "Return", "Space", "Backspace", "PrintScr"
- Arrow keys: "UpArrow", "DownArrow", "LeftArrow", "RightArrow"
- Modifier keys: "Alt", "Control", "Shift", "Meta", "Option", "CapsLock"
- Media keys: "VolumeUp", "VolumeDown", "VolumeMute", "MediaPlayPause", "MediaNextTrack", "MediaPrevTrack"
- On Windows: Numpad keys like "Numpad0", "Numpad1", etc. and "Num0", "Num1", etc.

### Examples (Consolidated)

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

Type the text "Example":
```bash
echo '{"event_type":"text","text":"Example"}' | ./key-listener SIMULATION
```

Move mouse instantly to position (100, 200):
```bash
echo '{"event_type":"mouse","action":"move","x":100,"y":200}' | ./key-listener SIMULATION
```

Move mouse smoothly to (500, 500) over 1 second using easeInOutQuad easing:
```bash
echo '{"event_type":"mouse","action":"move","x":500,"y":500,"duration_ms":1000,"ease":"easeInOutQuad"}' | ./key-listener SIMULATION
```

Click the left mouse button:
```bash
echo '{"event_type":"mouse","action":"click","button":"left"}' | ./key-listener SIMULATION
```

Scroll down 10 units instantly:
```bash
echo '{"event_type":"mouse","action":"scroll","delta_y":-10}' | ./key-listener SIMULATION
```

Scroll up 20 units smoothly over 300ms:
```bash
echo '{"event_type":"mouse","action":"scroll","delta_y":20,"duration_ms":300,"ease":"linear"}' | ./key-listener SIMULATION
```

Drag the mouse from (100, 100) to (300, 300):
```bash
echo '{"event_type":"mouse","action":"move","x":100,"y":100}' | ./key-listener SIMULATION # Move to start
echo '{"event_type":"mouse","action":"press","button":"left","delay_after_ms":50}' | ./key-listener SIMULATION # Press left button
echo '{"event_type":"mouse","action":"move","x":300,"y":300,"duration_ms":500,"ease":"linear"}' | ./key-listener SIMULATION # Move smoothly
echo '{"event_type":"mouse","action":"release","button":"left"}' | ./key-listener SIMULATION # Release button
```

## Mouse Handling Mode

When in mouse handling mode, the program tracks mouse movements and button events and outputs them as JSON:

```json
{"event_type":"move","x":512,"y":384,"timestamp":1620000000000}
{"event_type":"button","x":512,"y":384,"button":"left","pressed":true,"timestamp":1620000000100}
{"event_type":"button","x":512,"y":384,"button":"left","pressed":false,"timestamp":1620000000200}
```

## License

MIT
