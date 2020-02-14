# Ombnibar

A GTK based Wayland statusbar written in Rust, inspired by [Waybar](https://github.com/Alexays/Waybar) and [Polybar](https://github.com/polybar/polybar).

Only works on Wayland compositors that implement the wlr-layer-shell protocol, which includes wlroots based compositors like Sway.

## Dependencies

- GTK3
- Wayland
- [gtk-layer-shell](https://github.com/wmww/gtk-layer-shell)
- libpulse [PulseAudio module]
- sway [Sway modules]

## Installation

### From source

```bash
cargo install --git https://github.com/cjbassi/omnibar
```
