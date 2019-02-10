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

// const Types: *const *const WlInterface = vec![
//     null(),
//     null(),
//     null(),
//     null(),
//     &WlrLayerSurfaceInterface,
//     &WlSurfaceInterface,
//     &WlOutputInterface,
//     null(),
//     null(),
//     &XdgPopupInterface,
// ]
// .as_mut_ptr();

// const WlrLayerSurfaceRequests: *const WlMessage = vec![
//     WlMessage {
//         name: CString::new("set_size").unwrap().as_ptr(),
//         signature: CString::new("uu").unwrap().as_ptr(),
//         types: Types,
//     },
//     WlMessage {
//         name: CString::new("set_anchor").unwrap().as_ptr(),
//         signature: CString::new("u").unwrap().as_ptr(),
//         types: Types,
//     },
//     WlMessage {
//         name: CString::new("set_exclusive_zone").unwrap().as_ptr(),
//         signature: CString::new("i").unwrap().as_ptr(),
//         types: Types,
//     },
//     WlMessage {
//         name: CString::new("set_margin").unwrap().as_ptr(),
//         signature: CString::new("iiii").unwrap().as_ptr(),
//         types: Types,
//     },
//     WlMessage {
//         name: CString::new("set_keyboard_interactivity").unwrap().as_ptr(),
//         signature: CString::new("u").unwrap().as_ptr(),
//         types: Types,
//     },
//     WlMessage {
//         name: CString::new("get_popup").unwrap().as_ptr(),
//         signature: CString::new("o").unwrap().as_ptr(),
//         types: Types.offset(9),
//     },
//     WlMessage {
//         name: CString::new("ack_configure").unwrap().as_ptr(),
//         signature: CString::new("u").unwrap().as_ptr(),
//         types: Types,
//     },
//     WlMessage {
//         name: CString::new("destroy").unwrap().as_ptr(),
//         signature: CString::new("").unwrap().as_ptr(),
//         types: Types,
//     },
// ]
// .as_ptr();

// const WlrLayerSurfaceEvents: *const WlMessage = vec![
//     WlMessage {
//         name: CString::new("configure").unwrap().as_ptr(),
//         signature: CString::new("uuu").unwrap().as_ptr(),
//         types: Types,
//     },
//     WlMessage {
//         name: CString::new("closed").unwrap().as_ptr(),
//         signature: CString::new("").unwrap().as_ptr(),
//         types: Types,
//     },
// ]
// .as_ptr();

// const WlrLayerSurfaceInterface: WlInterface = WlInterface {
//     name: CString::new("zwlr_layer_surface_v1").unwrap().as_ptr(),
//     version: 1,
//     method_count: 8,
//     methods: WlrLayerSurfaceRequests,
//     event_count: 2,
//     events: WlrLayerSurfaceEvents,
// };

// // #[link(name = "libclient_protos")]
// // extern "C" {
// //     #[no_mangle]
// //     static wlr_layer_surface_interface: WlInterface;
// // }
