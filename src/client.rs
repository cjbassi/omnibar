use crate::bar;
use crate::bindings::gtk::{gdk_display_get_default, gdk_init, gdk_wayland_display_get_wl_display};
use crate::bindings::wayland::{
    wl_display_dispatch, wl_display_get_registry, wl_display_roundtrip, wl_registry_add_listener,
    wl_registry_bind, WlOutput, WlOutputInterface, WlRegistry, WlRegistryListener,
};
use crate::bindings::wlr::WlrLayerShell;
use libc::{c_void, uint32_t};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::process::exit;
use std::ptr::null_mut;

#[repr(C)]
pub struct Client {
    pub wlr_layer_shell: *mut WlrLayerShell,
    // gdk_display: *mut GdkDisplay,
    // wl_display: *mut WlDisplay,
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
        let mut client = Client {
            wlr_layer_shell: null_mut(),
        };
        let error = wl_registry_add_listener(
            registry,
            &WL_REGISTRY_LISTENER as *const WlRegistryListener,
            &mut client as *mut _ as *mut c_void,
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

        // Client {
        // gdk_display,
        // wl_display,
        // };
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
    let client = data as *mut Client;
    let interface = unsafe { CStr::from_ptr(interface) }.to_str().unwrap();
    match interface {
        "zwlr_layer_shell_v1" => {
            // (*data).layer_shell = unsafe { wl_registry_bind(registry, name, interface, version) } as *mut zwlr_layer_shell_v1;
        }
        "wl_output" => {
            let output = unsafe { wl_registry_bind(registry, name, &WlOutputInterface, version) };
            bar::Bar::new(unsafe { &mut *client }, output as *mut WlOutput);
        }
        "wl_seat" => {}
        "zxdg_output_manager_v1" => {}
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn handle_global_remove(
    _data: *mut c_void,
    _registry: *mut WlRegistry,
    _name: uint32_t,
) {
}

pub const WL_REGISTRY_LISTENER: WlRegistryListener = WlRegistryListener {
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
