#![allow(dead_code)]

use crate::bindings::wayland::{
    wl_proxy_add_listener, wl_proxy_marshal_constructor, WlInterface, WlOutput, WlProxy,
};

use libc::c_void;

use std::os::raw::{c_int, c_uint};
use std::ptr::null_mut;

pub const ZXDG_OUTPUT_V1_NAME_SINCE_VERSION: c_uint = 2;

const ZXDG_OUTPUT_MANAGER_V1_GET_XDG_OUTPUT: c_uint = 1;

pub enum XdgOutput {}
pub enum XdgOutputManager {}

#[repr(C)]
pub struct XdgOutputListener {
    pub logical_position: *const c_void,
    pub logical_size: *const c_void,
    pub done: *const c_void,
    pub name: *const c_void,
    pub description: *const c_void,
}

#[link(name = "client_protos")]
extern "C" {
    #[no_mangle]
    pub static zxdg_output_manager_v1_interface: WlInterface;
    #[no_mangle]
    static zxdg_output_v1_interface: WlInterface;
}

pub unsafe fn zxdg_output_manager_v1_get_xdg_output(
    manager: *mut XdgOutputManager,
    output: *mut WlOutput,
) -> *mut XdgOutput {
    wl_proxy_marshal_constructor(
        manager as *mut WlProxy,
        ZXDG_OUTPUT_MANAGER_V1_GET_XDG_OUTPUT,
        &zxdg_output_v1_interface,
        null_mut::<WlProxy>(),
        output,
    ) as *mut XdgOutput
}

pub unsafe fn zxdg_output_v1_add_listener(
    output: *mut XdgOutput,
    listener: *const XdgOutputListener,
    data: *mut c_void,
) -> c_int {
    wl_proxy_add_listener(output as *mut WlProxy, listener as *const c_void, data)
}
