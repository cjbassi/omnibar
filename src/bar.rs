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
use crate::client::CLIENT;
use crate::NAME;
use gtk_sys::{
    gtk_widget_get_window, gtk_widget_realize, gtk_widget_set_size_request, gtk_widget_show_all,
    gtk_window_new, gtk_window_resize, GtkWidget, GtkWindow, GTK_WINDOW_TOPLEVEL,
};
use lazy_static::lazy_static;
use libc::{c_char, c_void, int32_t, uint32_t};
use std::ffi::CString;
use std::process::exit;
use std::ptr::null_mut;
use std::sync::Mutex;

lazy_static! {
    static ref BAR: Mutex<Bar> = Mutex::new(Bar {
        widget: null_mut(),
        surface: null_mut(),
        output: null_mut()
    });
}

unsafe impl Send for Bar {}

pub struct Bar {
    widget: *mut GtkWidget,
    surface: *mut WlSurface,
    output: *mut WlOutput,
}

impl Bar {
    pub fn new(wl_output: *mut WlOutput) {
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

        {
            let mut bar = BAR.lock().unwrap();
            bar.widget = gtk_widget_ptr;
            bar.surface = wl_surface_ptr;
            bar.output = wl_output;
        }

        let xdg_output = unsafe {
            zxdg_output_manager_v1_get_xdg_output(
                CLIENT.lock().unwrap().xdg_output_manager,
                wl_output,
            )
        };
        if xdg_output.is_null() {
            exit(1);
        }
        unsafe {
            zxdg_output_v1_add_listener(
                xdg_output,
                &XDG_OUTPUT_LISTENER as *const XdgOutputListener,
                null_mut(),
            )
        };
    }

    fn init(&mut self) {
        let width = 1920;
        let height = 50;

        let layer_surface = unsafe {
            wlr_layer_shell_get_layer_surface(
                CLIENT.lock().unwrap().wlr_layer_shell,
                self.surface,
                self.output,
                WlrLayerShellLayer::Top,
                CString::new(NAME).unwrap().as_ptr(),
            )
        };

        unsafe {
            zwlr_layer_surface_v1_add_listener(
                layer_surface,
                &WLR_LAYER_SURFACE_LISTENER as *const WlrLayerSurfaceListener,
                null_mut(),
            )
        };

        let anchor = WlrLayerSurfaceAnchor::Left
            | WlrLayerSurfaceAnchor::Right
            | WlrLayerSurfaceAnchor::Bottom;
        unsafe { zwlr_layer_surface_v1_set_anchor(layer_surface, anchor) };
        unsafe { zwlr_layer_surface_v1_set_exclusive_zone(layer_surface, height) };
        unsafe { zwlr_layer_surface_v1_set_size(layer_surface, width, height as u32) };

        unsafe { wl_surface_commit(self.surface) };

        self.setup_widgets();
    }

    fn setup_widgets(&self) {
        unsafe { gtk_widget_show_all(self.widget) };
    }

    fn get_modules(&self) {}
}

#[no_mangle]
pub extern "C" fn xdg_handle_logical_position(
    _data: *mut c_void,
    _xdg_output: *mut XdgOutput,
    _x: int32_t,
    _y: int32_t,
) {
}

#[no_mangle]
pub extern "C" fn xdg_handle_logical_size(
    _data: *mut c_void,
    _xdg_output: *mut XdgOutput,
    _width: int32_t,
    _height: int32_t,
) {
}

#[no_mangle]
pub extern "C" fn xdg_handle_done(_data: *mut c_void, _xdg_output: *mut XdgOutput) {}

#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn xdg_handle_name(
    _data: *mut c_void,
    _xdg_output: *mut XdgOutput,
    _name: *const c_char,
) {
    let mut bar = BAR.lock().unwrap();
    bar.init();
}

#[no_mangle]
pub extern "C" fn xdg_handle_description(
    _data: *mut c_void,
    _xdg_output: *mut XdgOutput,
    _description: *const c_char,
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
    _data: *mut libc::c_void,
    surface: *mut WlrLayerSurface,
    serial: libc::uint32_t,
    w: uint32_t,
    h: uint32_t,
) {
    let bar = BAR.lock().unwrap();
    unsafe { zwlr_layer_surface_v1_ack_configure(surface, serial) };
    unsafe { gtk_widget_set_size_request(bar.widget, w as int32_t, h as int32_t) };
    unsafe { gtk_window_resize(bar.widget as *mut GtkWindow, w as int32_t, h as int32_t) };
}

#[no_mangle]
pub extern "C" fn wlr_layer_surface_closed(
    _data: *mut libc::c_void,
    _surface: *mut WlrLayerSurface,
) {
}

pub const WLR_LAYER_SURFACE_LISTENER: WlrLayerSurfaceListener = WlrLayerSurfaceListener {
    configure: wlr_layer_surface_configure as *const _,
    closed: wlr_layer_surface_closed as *const _,
};
