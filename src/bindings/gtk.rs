#![allow(dead_code)]

use crate::bindings::wayland::{WlDisplay, WlSurface};
use std::os::raw::{c_char, c_int};

pub enum GdkDisplay {}
pub enum GdkWindow {}

pub enum GtkWidget {}

#[repr(C)]
pub enum GtkWindowType {
    TopLevel,
    Popup,
}

#[link(name = "gdk-3")]
extern "C" {
    pub fn gdk_init(argc: *mut c_int, argv: *mut c_char);
    pub fn gdk_display_get_default() -> *mut GdkDisplay;

    pub fn gdk_wayland_display_get_wl_display(display: *mut GdkDisplay) -> *mut WlDisplay;

    pub fn gdk_wayland_window_get_wl_surface(window: *mut GdkWindow) -> *mut WlSurface;
    pub fn gdk_wayland_window_set_use_custom_surface(window: *mut GdkWindow);
}

#[link(name = "gtk-3")]
extern "C" {
    pub fn gtk_main();

    pub fn gtk_window_new(type_: GtkWindowType) -> *mut GtkWidget;
    pub fn gtk_widget_realize(widget: *mut GtkWidget);
    pub fn gtk_widget_get_window(widget: *mut GtkWidget) -> *mut GdkWindow;
    pub fn gtk_widget_show_all(widget: *mut GtkWidget);
    pub fn gtk_widget_set_size_request(widget: *mut GtkWidget, width: c_int, height: c_int);
}
