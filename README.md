# Redmond Desktop Environment

A long time ago, in a computer far away, there was an operating system. This looks like it, but is
built on much stronger foundations.

Start:

```
cargo run -- --x11 &
sleep 1 && WAYLAND_DISPLAY=wayland-1 cargo run --bin rde-panel &
sleep 2 && WAYLAND_DISPLAY=wayland-1 cargo run --bin rde-calc
```

## License

The project as a whole is licensed under the AGPL-3.0-or-later.

Parts may contain code licensed under additional compatible licenses.
