<div style="display: flex; justify-content: center; align-items: center;">
  <img src="assets/icon.png" alt="image" />
</div>

# GPad View

A Windows gamepad/controller inspector built with Rust.

## Demo

https://github.com/user-attachments/assets/a4084fe7-5427-4035-ac36-cba75eecfc3d

## Requirements

- Windows 10 or later
- A connected game controller

## Installation

Soon: Download the latest release from the [releases page](#) and run `gpadview.exe`.

## Building from Source

### Prerequisites

- [Rust](https://rustup.rs) (stable)
- Windows SDK

### Steps
```bash
git clone https://github.com/yourname/gpadview
cd gpadview
cargo build --release
```

The compiled binary will be at `target/release/gpadview.exe`.

## Usage

1. Plug in a controller
2. Launch GPadView
3. Click on a controller card to inspect its inputs
4. Axes, buttons, and switches update in real time

## Tech Stack

- [Rust](https://www.rust-lang.org)
- [egui](https://github.com/emilk/egui) — immediate mode GUI
- [Windows Gaming Input](https://learn.microsoft.com/en-us/uwp/api/windows.gaming.input) — controller API
- [hidapi](https://github.com/ruabmbua/hidapi-rs) — device name lookup

## License
TODO
