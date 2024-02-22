# Olympus

## GUI for the robotics_lib

### Installation

#### Prerequisites

Macroquad has some OS dependencies, you need to install some packages:

Refer to Macroquad repo [README](https://github.com/not-fl3/macroquad?tab=readme-ov-file) for instruction on other platforms.

##### Linux

```
# ubuntu system dependencies
apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev

# fedora system dependencies
dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel

# arch linux system dependencies
 pacman -S pkg-config libx11 libxi mesa-libgl alsa-lib
```

##### Windows

No additional dependencies required.

#### Cargo

Add olympus as a dependency to Cargo.toml:

```toml
[dependencies]
olympus = { version = "2.0.0", git = "https://github.com/Kalsifer-742/olympus.git", tag="v2.0.0" }
```

In this way you will be always synced to the latest release.

### Documentation

To get the full documentation run `cargo doc` or `cargo doc --open` to automatically open the generated documentation in your browser.

> At the moment the documentation is the provided example

### Examples

Inside the folder run in your terminal: `cargo run --release --example example`.

The code is under `/examples/example.rs` .

### Notes

World generator for robotics_lib: [midgard](https://github.com/Kalsifer-742/midgard).
