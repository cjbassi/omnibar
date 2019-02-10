# ombnibar

A wlroots based Wayland statusbar written in Rust, largely inspired by [Waybar](https://github.com/Alexays/Waybar).

## Dependencies

- gkt3
- wayland
- wlroots
- sway [Sway module]

Uses FFI to call the C libraries.

## Installation

### Source

```bash
cargo install --git https://github.com/cjbassi/omnibar
```

### Prebuilt binaries

Run the following to download the correct binary for your system from the releases tab into $CARGO_HOME/bin: (currently only Linux-x86_64 is available)

```bash
bash <(curl https://raw.githubusercontent.com/japaric/trust/c268696ab9f054e1092f195dddeead2420c04261/install.sh) -f --git cjbassi/omnibar
```

## Configuration

Check out the [wiki](https://github.com/cjbassi/omnibar/wiki) for info on how to customize and style omnibar and for a list of available modules.
