use crate::bindings::gdk_wayland::{
    self, gdk_wayland_window_get_wl_surface, gdk_wayland_window_set_use_custom_surface,
};
use crate::bindings::wayland::{wl_surface_commit, WlOutput, WlSurface};
use crate::bindings::wlr::{
    wlr_layer_shell_get_layer_surface, zwlr_layer_surface_v1_ack_configure,
    zwlr_layer_surface_v1_add_listener, zwlr_layer_surface_v1_set_anchor,
    zwlr_layer_surface_v1_set_exclusive_zone, zwlr_layer_surface_v1_set_size, WlrLayerShellLayer,
    WlrLayerSurface, WlrLayerSurfaceAnchor, WlrLayerSurfaceListener,
};
use crate::bindings::xdg::{
    zxdg_output_manager_v1_get_xdg_output, zxdg_output_v1_add_listener, XdgOutput,
    XdgOutputListener,
};
use crate::client::Client;
use crate::NAME;
use gtk_sys::{
    gtk_button_new_with_label, gtk_container_add, gtk_widget_get_window, gtk_widget_realize,
    gtk_widget_set_size_request, gtk_widget_show_all, gtk_window_new, gtk_window_resize, GtkWidget,
    GtkWindow, GTK_WINDOW_TOPLEVEL,
};
use libc::{c_char, c_void, int32_t, uint32_t};
use std::ffi::CString;
use std::process::exit;

pub struct Bar {
    client: *mut Client,
    widget: *mut GtkWidget,
    surface: *mut WlSurface,
    output: *mut WlOutput,
}

impl Bar {
    pub fn new(client: &mut Client, wl_output: *mut WlOutput) -> Bar {
        let gtk_widget_ptr = unsafe { gtk_window_new(GTK_WINDOW_TOPLEVEL) };
        unsafe { gtk_widget_realize(gtk_widget_ptr) };
        let gdk_window_ptr = unsafe { gtk_widget_get_window(gtk_widget_ptr) };
        if gdk_window_ptr.is_null() {
            eprintln!("failed to get window of gtk widget");
            exit(1);
        }
        unsafe {
            gdk_wayland_window_set_use_custom_surface(gdk_window_ptr as *mut gdk_wayland::GdkWindow)
        };
        let wl_surface_ptr = unsafe {
            gdk_wayland_window_get_wl_surface(gdk_window_ptr as *mut gdk_wayland::GdkWindow)
        };

        let mut bar = Bar {
            client: client as *mut Client,
            widget: gtk_widget_ptr,
            surface: wl_surface_ptr,
            output: wl_output,
        };

        let xdg_output =
            unsafe { zxdg_output_manager_v1_get_xdg_output(client.xdg_output_manager, wl_output) };
        if xdg_output.is_null() {
            exit(1);
        }
        unsafe {
            zxdg_output_v1_add_listener(
                xdg_output,
                &XDG_OUTPUT_LISTENER as *const XdgOutputListener,
                &mut bar as *mut _ as *mut c_void,
            )
        };
        bar
    }

    fn init(&mut self) {
        let width = 1920;
        let height = 50;

        let layer_surface = unsafe {
            wlr_layer_shell_get_layer_surface(
                (*self.client).wlr_layer_shell,
                self.surface,
                self.output,
                WlrLayerShellLayer::Top,
                CString::new(NAME).unwrap().as_ptr(),
            )
        };

        // unsafe {
        //     zwlr_layer_surface_v1_add_listener(
        //         layer_surface,
        //         &WLR_LAYER_SURFACE_LISTENER as *const WlrLayerSurfaceListener,
        //         self as *mut _ as *mut c_void,
        //     )
        // };

        // let anchor = WlrLayerSurfaceAnchor::Left
        //     | WlrLayerSurfaceAnchor::Right
        //     | WlrLayerSurfaceAnchor::Bottom;
        // unsafe { zwlr_layer_surface_v1_set_anchor(layer_surface, anchor) };
        // unsafe { zwlr_layer_surface_v1_set_exclusive_zone(layer_surface, height) };
        // unsafe { zwlr_layer_surface_v1_set_size(layer_surface, width, height as u32) };

        // unsafe { wl_surface_commit(self.surface) };

        // self.setup_widgets();
    }

    fn setup_widgets(&self) {
        unsafe { gtk_widget_show_all(self.widget) };
    }

    fn get_modules(&self) {}
}

#[no_mangle]
pub extern "C" fn xdg_handle_logical_position(
    data: *mut c_void,
    xdg_output: *mut XdgOutput,
    x: int32_t,
    y: int32_t,
) {
}

#[no_mangle]
pub extern "C" fn xdg_handle_logical_size(
    data: *mut c_void,
    xdg_output: *mut XdgOutput,
    width: int32_t,
    height: int32_t,
) {
}

#[no_mangle]
pub extern "C" fn xdg_handle_done(data: *mut c_void, xdg_output: *mut XdgOutput) {}

#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn xdg_handle_name(
    data: *mut c_void,
    xdg_output: *mut XdgOutput,
    name: *const c_char,
) {
    if data.is_null() {
        exit(1);
    }
    let bar_ptr = data as *mut Bar;
    unsafe { (*bar_ptr).init() };
}

#[no_mangle]
pub extern "C" fn xdg_handle_description(
    data: *mut c_void,
    xdg_output: *mut XdgOutput,
    description: *const c_char,
) {
}

pub const XDG_OUTPUT_LISTENER: XdgOutputListener = XdgOutputListener {
    logical_position: xdg_handle_logical_position as *const _,
    logical_size: xdg_handle_logical_size as *const _,
    done: xdg_handle_done as *const _,
    name: xdg_handle_name as *const _,
    description: xdg_handle_description as *const _,
};

#[no_mangle]
pub extern "C" fn wlr_layer_surface_configure(
    data: *mut libc::c_void,
    surface: *mut WlrLayerSurface,
    serial: libc::uint32_t,
    w: uint32_t,
    h: uint32_t,
) {
    let bar_ptr = data as *mut Bar;
    unsafe {
        zwlr_layer_surface_v1_ack_configure((*bar_ptr).surface as *mut WlrLayerSurface, serial)
    };
    unsafe { gtk_widget_set_size_request((*bar_ptr).widget, w as int32_t, h as int32_t) };
    unsafe {
        gtk_window_resize(
            (*bar_ptr).widget as *mut GtkWindow,
            w as int32_t,
            h as int32_t,
        )
    };
}

#[no_mangle]
pub extern "C" fn wlr_layer_surface_closed(data: *mut libc::c_void, surface: *mut WlrLayerSurface) {
}

pub const WLR_LAYER_SURFACE_LISTENER: WlrLayerSurfaceListener = WlrLayerSurfaceListener {
    configure: wlr_layer_surface_configure as *const _,
    closed: wlr_layer_surface_closed as *const _,
};

// unsafe {
//     gtk_sys::gtk_window_set_default_size(
//         gtk_widget_ptr as *mut gtk_sys::GtkWindow,
//         1920,
//         50,
//     )
// };

// let button = unsafe { gtk_button_new_with_label(CString::new("hello").unwrap().as_ptr()) };
// unsafe { gtk_container_add(gtk_widget_ptr as *mut gtk_sys::GtkContainer, button) };

// unsafe { wl_surface_commit(wl_surface_ptr) };

// unsafe { wl_surface_commit(wl_surface_ptr) };
