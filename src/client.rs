use crate::gdk::{
    gdk_display_get_default, gdk_init, gdk_wayland_display_get_wl_display, GdkDisplay,
};
use crate::wayland::{
    wl_display_dispatch, wl_display_get_registry, wl_display_roundtrip, wl_registry_add_listener,
    WlRegistryListener,
};
use crate::wayland::{WlDisplay, WlRegistry};
use libc::{c_void, uint32_t};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::process::exit;
use std::ptr::null_mut;

pub struct Client {
    gdk_display: *mut GdkDisplay,
    wl_display: *mut WlDisplay,
}

impl Client {
    pub fn new() {
        unsafe { gdk_init(null_mut(), null_mut()) };
        let gdk_display = unsafe { gdk_display_get_default() };
        let wl_display = unsafe { gdk_wayland_display_get_wl_display(gdk_display) };
        let registry = wl_display_get_registry(wl_display);
        if registry.is_null() {
            eprintln!("failed to get registry");
            exit(1);
        }
        let error = wl_registry_add_listener(
            registry,
            &REGISTRY_LISTENER as *const WlRegistryListener,
            null_mut(),
        );
        if error == -1 {
            eprintln!("failed to add registry_listener");
            exit(1);
        }
        let error = unsafe { wl_display_dispatch(wl_display) };
        if error == -1 {
            eprintln!("failed display_dispatch");
            exit(1);
        }
        let error = unsafe { wl_display_roundtrip(wl_display) };
        if error == -1 {
            eprintln!("failed display roundtrip");
            exit(1);
        }

        Client {
            gdk_display,
            wl_display,
        };
    }
}

#[no_mangle]
pub extern "C" fn handle_global(
    data: *mut libc::c_void,
    registry: *mut WlRegistry,
    name: libc::uint32_t,
    interface: *const c_char,
    version: libc::uint32_t,
) {
    let interface = unsafe { CStr::from_ptr(interface) }.to_str().unwrap();
    match interface {
        "zwlr_layer_shell_v1" => {
            println!("hi");
        }
        "wl_output" => {}
        "wl_seat" => {}
        "zxdg_output_manager_v1" => {}
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn handle_global_remove(
    data: *mut c_void,
    registry: *mut WlRegistry,
    name: uint32_t,
) {
}

pub const REGISTRY_LISTENER: WlRegistryListener = WlRegistryListener {
    global: handle_global as *const _,
    global_remove: handle_global_remove as *const _,
};

// gdk::init();
// gdk::Display::get_default();
// let display = unsafe { wl_display_connect(null()) };
// if display.is_null() {
//     eprintln!("failed to create display");
//     exit(1);
// }
