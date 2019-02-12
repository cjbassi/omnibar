use crate::bar;
use crate::bindings::gdk_wayland::{self, gdk_wayland_display_get_wl_display};
use crate::bindings::wayland::{
    wl_display_dispatch, wl_display_get_registry, wl_display_roundtrip, wl_output_interface,
    wl_registry_add_listener, wl_registry_bind, WlOutput, WlRegistry, WlRegistryListener,
};
use crate::bindings::wlr::{zwlr_layer_shell_v1_interface, WlrLayerShell};
use crate::bindings::xdg::{
    zxdg_output_manager_v1_interface, XdgOutputManager, ZXDG_OUTPUT_V1_NAME_SINCE_VERSION,
};
use crate::{check_error, check_null};

use gdk_sys::{gdk_display_get_default, gdk_init};
use gtk_sys::gtk_main;

use lazy_static::lazy_static;

use libc::{c_void, uint32_t};

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr::null_mut;
use std::sync::Mutex;

lazy_static! {
    pub static ref CLIENT: Mutex<Client> = Mutex::new(Client {
        wlr_layer_shell: null_mut(),
        xdg_output_manager: null_mut(),
    });
}

#[repr(C)]
pub struct Client {
    pub wlr_layer_shell: *mut WlrLayerShell,
    pub xdg_output_manager: *mut XdgOutputManager,
}

unsafe impl Send for Client {}

pub fn init() {
    unsafe { gdk_init(null_mut(), null_mut()) };
    let gdk_display = unsafe { gdk_display_get_default() };
    check_null!(gdk_display);
    let wl_display =
        unsafe { gdk_wayland_display_get_wl_display(gdk_display as *mut gdk_wayland::GdkDisplay) };
    check_null!(wl_display);
    let registry = unsafe { wl_display_get_registry(wl_display) };
    check_null!(registry);
    let error = unsafe {
        wl_registry_add_listener(
            registry,
            &WL_REGISTRY_LISTENER as *const WlRegistryListener,
            null_mut(),
        )
    };
    check_error!(error);
    let error = unsafe { wl_display_dispatch(wl_display) };
    check_error!(error);
    let error = unsafe { wl_display_roundtrip(wl_display) };
    check_error!(error);
    unsafe { gtk_main() };
}

#[no_mangle]
pub extern "C" fn wl_handle_global(
    _data: *mut libc::c_void,
    registry: *mut WlRegistry,
    name: libc::uint32_t,
    interface: *const c_char,
    version: libc::uint32_t,
) {
    check_null!(registry);
    check_null!(interface);
    let interface = unsafe { CStr::from_ptr(interface) }.to_str().unwrap();
    match interface {
        "zwlr_layer_shell_v1" => {
            CLIENT.lock().unwrap().wlr_layer_shell = unsafe {
                wl_registry_bind(registry, name, &zwlr_layer_shell_v1_interface, version)
            } as *mut WlrLayerShell;
        }
        "wl_output" => {
            let output = unsafe { wl_registry_bind(registry, name, &wl_output_interface, version) };
            check_null!(output);
            bar::init(output as *mut WlOutput);
        }
        "wl_seat" => {}
        "zxdg_output_manager_v1" => {
            CLIENT.lock().unwrap().xdg_output_manager = unsafe {
                wl_registry_bind(
                    registry,
                    name,
                    &zxdg_output_manager_v1_interface,
                    ZXDG_OUTPUT_V1_NAME_SINCE_VERSION,
                )
            } as *mut XdgOutputManager;
        }
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn wl_handle_global_remove(
    _data: *mut c_void,
    _registry: *mut WlRegistry,
    _name: uint32_t,
) {
}

pub const WL_REGISTRY_LISTENER: WlRegistryListener = WlRegistryListener {
    global: wl_handle_global as *const _,
    global_remove: wl_handle_global_remove as *const _,
};
