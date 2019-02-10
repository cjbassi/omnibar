use crate::bindings::gtk::{
    gdk_wayland_window_get_wl_surface, gdk_wayland_window_set_use_custom_surface,
    gtk_widget_get_window, gtk_widget_realize, gtk_window_new, GtkWindowType,
};
//  gtk_widget_set_size_request, gtk_widget_show_all,
use crate::bindings::wayland::{wl_surface_commit, WlOutput};
use crate::bindings::wlr::{
    wlr_layer_shell_get_layer_surface, wlr_layer_surface_set_anchor,
    wlr_layer_surface_set_exclusive_zone, WlrLayerShellLayer, WlrLayerSurfaceAnchor,
};
use crate::client::Client;
use crate::NAME;
use std::ffi::CString;
use std::process::exit;

pub struct Bar {}

impl Bar {
    pub fn new(client: &mut Client, wl_output: *mut WlOutput) {
        let gtk_widget_ptr = unsafe { gtk_window_new(GtkWindowType::TopLevel) };
        unsafe { gtk_widget_realize(gtk_widget_ptr) };
        let gdk_window_ptr = unsafe { gtk_widget_get_window(gtk_widget_ptr) };
        if gdk_window_ptr.is_null() {
            eprintln!("failed to get window of gtk widget");
            exit(1);
        }
        unsafe { gdk_wayland_window_set_use_custom_surface(gdk_window_ptr) };
        let wl_surface_ptr = unsafe { gdk_wayland_window_get_wl_surface(gdk_window_ptr) };

        let height = 50;
        let layer_surface = wlr_layer_shell_get_layer_surface(
            client.wlr_layer_shell,
            wl_surface_ptr,
            wl_output,
            WlrLayerShellLayer::Top,
            CString::new(NAME).unwrap().as_ptr(),
        );
        let anchor = WlrLayerSurfaceAnchor::Left
            | WlrLayerSurfaceAnchor::Right
            | WlrLayerSurfaceAnchor::Bottom;
        wlr_layer_surface_set_anchor(layer_surface, anchor);
        wlr_layer_surface_set_exclusive_zone(layer_surface, height);

        unsafe { wl_surface_commit(wl_surface_ptr) };

        // unsafe { gtk_widget_set_size_request(gtk_widget_ptr, 100, 100) };
        // unsafe { gtk_widget_show_all(gtk_widget_ptr) };
    }
}
