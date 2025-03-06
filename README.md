# tinypicker
A color picker made in rust for the CLI!

When you pick a color it will both display the rgb hex of the color and put it into your clipboard as well as having the display background become the color and the foreground become it's inverse.

You can install it by running `cargo install tinypicker`

## Platform support:
- [x] Windows
- [x] MacOS
- [x] Linux (X11)
    - Preferably used with `xclip`/`xsel` installed
- [ ] Linux (Wayland)

## Cargo Features
Mouse Tracking:
- [device_query](https://crates.io/crates/device_query) (default)
- [mouce](https://crates.io/crates/mouce)

# Changelog

## 0.3.1

Fix multi-monitor color picks and follow `NO_COLOR` when set.

## 0.3.1

Updated dependencies and turned off tty colors when not being displayed in terminal directly.