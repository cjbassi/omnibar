#![allow(dead_code)]

use crate::bindings::wayland::{WlDisplay, WlSurface};

pub enum GdkDisplay {}
pub enum GdkWindow {}

#[link(name = "gdk-3")]
extern "C" {
    pub fn gdk_wayland_display_get_wl_display(display: *mut GdkDisplay) -> *mut WlDisplay;

    pub fn gdk_wayland_window_get_wl_surface(window: *mut GdkWindow) -> *mut WlSurface;
    pub fn gdk_wayland_window_set_use_custom_surface(window: *mut GdkWindow);
}
