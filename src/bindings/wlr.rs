#![allow(dead_code)]

use crate::bindings::wayland::{
    wl_proxy_marshal, wl_proxy_marshal_constructor, WlInterface, WlOutput, WlProxy, WlSurface,
};
use bitflags::bitflags;
use libc::int32_t;
use std::os::raw::{c_char, c_uint};
use std::ptr::null_mut;

pub enum WlrLayerShell {}
pub enum WlrLayerSurface {}

const WLR_LAYER_SHELL_GET_LAYER_SURFACE: c_uint = 0;

const WLR_LAYER_SURFACE_SET_ANCHOR: c_uint = 1;
const WLR_LAYER_SURFACE_SET_EXCLUSIVE_ZONE: c_uint = 2;

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

pub fn wlr_layer_shell_get_layer_surface(
    wlr_layer_shell: *mut WlrLayerShell,
    surface: *mut WlSurface,
    output: *mut WlOutput,
    layer: WlrLayerShellLayer,
    np: *const c_char,
) -> *mut WlrLayerSurface {
    let proxy: *mut WlProxy;

    unsafe {
        proxy = wl_proxy_marshal_constructor(
            wlr_layer_shell as *mut _ as *mut WlProxy,
            WLR_LAYER_SHELL_GET_LAYER_SURFACE,
            &zwlr_layer_surface_v1_interface,
            null_mut::<WlProxy>(),
            surface,
            output,
            layer,
            np,
        );
        proxy as *mut _ as *mut WlrLayerSurface
    }
}

pub fn wlr_layer_surface_set_anchor(
    wlr_layer_surface: *mut WlrLayerSurface,
    anchor: WlrLayerSurfaceAnchor,
) {
    unsafe {
        wl_proxy_marshal(
            wlr_layer_surface as *mut WlProxy,
            WLR_LAYER_SURFACE_SET_ANCHOR,
            anchor,
        )
    };
}

pub fn wlr_layer_surface_set_exclusive_zone(
    wlr_layer_surface: *mut WlrLayerSurface,
    zone: int32_t,
) {
    unsafe {
        wl_proxy_marshal(
            wlr_layer_surface as *mut WlProxy,
            WLR_LAYER_SURFACE_SET_EXCLUSIVE_ZONE,
            zone,
        )
    };
}
