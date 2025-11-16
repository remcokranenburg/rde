#![warn(rust_2018_idioms)]
// If no backend is enabled, a large portion of the codebase is unused.
// So silence this useless warning for the CI.
#![cfg_attr(
    not(any(feature = "winit", feature = "udev")),
    allow(dead_code, unused_imports)
)]

#[cfg(any(feature = "udev", feature = "xwayland"))]
pub mod cursor;
pub mod drawing;
pub mod focus;
pub mod input_handler;
pub mod render;
pub mod shell;
pub mod state;
#[cfg(feature = "udev")]
pub mod udev;
#[cfg(feature = "winit")]
pub mod winit;

use std::{
    io,
    process::{Child, Command},
};

pub use state::{AnvilState, ClientState};

pub fn run_dev_panel(wayland_socket: &str) -> Result<Child, io::Error> {
    Command::new("cargo")
        .env("WAYLAND_DISPLAY", wayland_socket)
        .arg("run")
        .arg("--bin")
        .arg("panel")
        .spawn()
}
