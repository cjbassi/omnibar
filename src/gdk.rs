use crate::wayland::WlDisplay;
use std::os::raw::{c_char, c_int};

pub enum GdkDisplay {}

#[link(name = "gdk-3")]
extern "C" {
    pub fn gdk_init(argc: *mut c_int, argv: *mut c_char);
    pub fn gdk_display_get_default() -> *mut GdkDisplay;
    pub fn gdk_wayland_display_get_wl_display(display: *mut GdkDisplay) -> *mut WlDisplay;
}
