# RDE WM

Based on Anvil, `wm` is a window manager that behaves like Windows.

## Dependencies

You'll need to install the following dependencies (note, that those package
names may vary depending on your OS and linux distribution):

- `libwayland`
- `libxkbcommon`

#### These are needed for the "Udev/DRM backend"

- `libudev`
- `libinput`
- `libgbm`
- [`libseat`](https://git.sr.ht/~kennylevinsen/seatd)

If you want to enable X11 support (to run X11 applications within anvil),
then you'll need to install the following packages as well:
    - `xwayland`

## Build and run

You can run it with cargo after having cloned this repository:

```
cargo run -- [--dev-panel]
```

The currently available backends are:

- `winit`: start wm as a window inside a running Wayland session. This happens automatically when
  WAYLAND_DISPLAY is set.

- `udev`: start wm in a tty with udev support. This is the "traditional" launch of a Wayland
  compositor, and will happen if WAYLAND_DISPLAY is not already set.

### Supported Environment Variables

| Variable                      | Example         | Backends  |
|-------------------------------|-----------------|-----------|
| ANVIL_DRM_DEVICE              | /dev/dri/card0  | tty-udev  |
| ANVIL_DISABLE_10BIT           | any             | tty-udev  |
| ANVIL_DISABLE_DIRECT_SCANOUT  | any             | tty-udev  |
| SMITHAY_USE_LEGACY            | 1,true,yes,y    | tty-udev  |
| SMITHAY_VK_VERSION            | 1.3             |           |
