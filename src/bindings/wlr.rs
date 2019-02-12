#![allow(dead_code)]

use crate::bindings::wayland::{
    wl_proxy_add_listener, wl_proxy_marshal, wl_proxy_marshal_constructor, WlInterface, WlOutput,
    WlProxy, WlSurface,
};
use bitflags::bitflags;
use libc::{int32_t, uint32_t};
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr::null_mut;

pub enum WlrLayerShell {}
pub enum WlrLayerSurface {}

const WLR_LAYER_SHELL_GET_LAYER_SURFACE: c_uint = 0;

const WLR_LAYER_SURFACE_SET_ANCHOR: c_uint = 1;
const WLR_LAYER_SURFACE_SET_EXCLUSIVE_ZONE: c_uint = 2;

const ZWLR_LAYER_SURFACE_V1_SET_SIZE: c_uint = 0;
const ZWLR_LAYER_SURFACE_V1_ACK_CONFIGURE: c_uint = 6;

#[repr(C)]
pub struct WlrLayerSurfaceListener {
    pub configure: *const c_void,
    pub closed: *const c_void,
}

#[link(name = "client_protos")]
extern "C" {
    #[no_mangle]
    pub static zwlr_layer_surface_v1_interface: WlInterface;
    #[no_mangle]
    pub static zwlr_layer_shell_v1_interface: WlInterface;
}

bitflags! {
    pub struct WlrLayerSurfaceAnchor: u32 {
        const Top = 1;
        const Bottom = 2;
        const Left = 4;
        const Right = 8;
    }
}

#[repr(C)]
pub enum WlrLayerShellLayer {
    Background = 0,
    Bottom = 1,
    Top = 2,
    Overlay = 3,
}

pub unsafe fn wlr_layer_shell_get_layer_surface(
    wlr_layer_shell: *mut WlrLayerShell,
    surface: *mut WlSurface,
    output: *mut WlOutput,
    layer: WlrLayerShellLayer,
    np: *const c_char,
) -> *mut WlrLayerSurface {
    dbg!(wlr_layer_shell);
    dbg!(surface);
    dbg!(output);
    dbg!(np);
    wl_proxy_marshal_constructor(
        wlr_layer_shell as *mut WlProxy,
        WLR_LAYER_SHELL_GET_LAYER_SURFACE,
        &zwlr_layer_surface_v1_interface,
        null_mut::<WlProxy>(),
        surface,
        output,
        layer,
        np,
    ) as *mut WlrLayerSurface
}

pub unsafe fn zwlr_layer_surface_v1_set_anchor(
    wlr_layer_surface: *mut WlrLayerSurface,
    anchor: WlrLayerSurfaceAnchor,
) {
    wl_proxy_marshal(
        wlr_layer_surface as *mut WlProxy,
        WLR_LAYER_SURFACE_SET_ANCHOR,
        anchor,
    )
}

pub unsafe fn zwlr_layer_surface_v1_set_exclusive_zone(
    wlr_layer_surface: *mut WlrLayerSurface,
    zone: int32_t,
) {
    wl_proxy_marshal(
        wlr_layer_surface as *mut WlProxy,
        WLR_LAYER_SURFACE_SET_EXCLUSIVE_ZONE,
        zone,
    )
}

pub unsafe fn zwlr_layer_surface_v1_ack_configure(surface: *mut WlrLayerSurface, serial: uint32_t) {
    wl_proxy_marshal(
        surface as *mut WlProxy,
        ZWLR_LAYER_SURFACE_V1_ACK_CONFIGURE,
        serial,
    );
}

pub unsafe fn zwlr_layer_surface_v1_set_size(
    surface: *mut WlrLayerSurface,
    width: uint32_t,
    height: uint32_t,
) {
    wl_proxy_marshal(
        surface as *mut WlProxy,
        ZWLR_LAYER_SURFACE_V1_SET_SIZE,
        width,
        height,
    );
}

pub unsafe fn zwlr_layer_surface_v1_add_listener(
    surface: *mut WlrLayerSurface,
    listener: *const WlrLayerSurfaceListener,
    data: *mut c_void,
) -> c_int {
    wl_proxy_add_listener(surface as *mut WlProxy, listener as *const c_void, data)
}
