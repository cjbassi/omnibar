// https://stackoverflow.com/questions/50708059/why-is-rust-unable-to-find-wl-display-get-registry

use libc::{c_void, uint32_t};
use std::os::raw::{c_char, c_int, c_uint};

const WL_DISPLAY_GET_REGISTRY: c_uint = 1;
const WL_SURFACE_COMMIT: c_uint = 6;

pub enum WlDisplay {}
pub enum WlOutput {}
pub enum WlSurface {}
pub enum WlRegistry {}
pub enum WlProxy {}

#[repr(C)]
pub struct WlRegistryListener {
    pub global: *const c_void,
    pub global_remove: *const c_void,
}

#[repr(C)]
pub struct WlMessage {
    name: *const c_char,
    signature: *const c_char,
    types: *const *const WlInterface,
}

#[repr(C)]
pub struct WlInterface {
    name: *const c_char,
    version: c_int,
    method_count: c_int,
    methods: *const WlMessage,
    event_count: c_int,
    events: *const WlMessage,
}

#[link(name = "wayland-client")]
extern "C" {
    #[no_mangle]
    pub static WlRegistryInterface: WlInterface;
    #[no_mangle]
    pub static WlOutputInterface: WlInterface;

    pub fn wl_display_connect(name: *const c_char) -> *mut WlDisplay;
    pub fn wl_display_disconnect(display: *mut WlDisplay);

    pub fn wl_display_dispatch(display: *mut WlDisplay) -> c_int;
    pub fn wl_display_roundtrip(display: *mut WlDisplay) -> c_int;

    pub fn wl_proxy_marshal(proxy: *mut WlProxy, opcode: uint32_t, ...);

    pub fn wl_registry_bind(
        wl_registry: *mut WlRegistry,
        name: uint32_t,
        interface: *const WlInterface,
        version: uint32_t,
    ) -> *mut c_void;

    pub fn wl_proxy_add_listener(
        proxy: *mut WlProxy,
        implementation: *const c_void,
        data: *mut c_void,
    ) -> c_int;

    pub fn wl_proxy_marshal_constructor(
        proxy: *mut WlProxy,
        opcode: c_uint,
        interface: *const WlInterface,
        ...
    ) -> *mut WlProxy;
}

pub unsafe fn wl_surface_commit(surface: *mut WlSurface) {
    wl_proxy_marshal(surface as *mut WlProxy, WL_SURFACE_COMMIT);
}

pub fn wl_registry_add_listener(
    wl_registry: *mut WlRegistry,
    listener: *const WlRegistryListener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        return wl_proxy_add_listener(
            wl_registry as *mut _ as *mut WlProxy,
            listener as *const _,
            data,
        );
    }
}

pub fn wl_display_get_registry(display: *mut WlDisplay) -> *mut WlRegistry {
    let proxy: *mut WlProxy;

    unsafe {
        proxy = wl_proxy_marshal_constructor(
            display as *mut _ as *mut WlProxy,
            WL_DISPLAY_GET_REGISTRY,
            &WlRegistryInterface,
        );
        proxy as *mut _ as *mut WlRegistry
    }
}

// #[repr(C)]
// pub struct WlOutputInterface {
//     pub release: *const c_void,
// }

// pub const WL_OUTPUT_INTERFACE: WlOutputInterface = WlOutputInterface {};

// #[repr(C)]
// pub struct WlRegistryListener {
//     global: *const Fn(*mut c_void, *mut WlRegistry, uint32_t, *const c_char, uint32_t),
//     global_remove: *const Fn(*mut c_void, *mut WlRegistry, uint32_t),
// }
