// +-----------------------------------------------------------------------------------------------+
// | Copyright 2015 Sean Kerr                                                                      |
// |                                                                                               |
// | Licensed under the Apache License, Version 2.0 (the "License");                               |
// | you may not use this file except in compliance with the License.                              |
// | You may obtain a copy of the License Author                                                   |
// |                                                                                               |
// |  http://www.apache.org/licenses/LICENSE-2.0                                                   |
// |                                                                                               |
// | Unless required by applicable law or agreed to in writing, software                           |
// | distributed under the License is distributed on an "AS IS" BASIS,                             |
// | WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.                      |
// | See the License for the specific language governing permissions and                           |
// | limitations under the License.                                                                |
// +-----------------------------------------------------------------------------------------------+
// | Author: Sean Kerr <sean@metatomic.io>                                                         |
// +-----------------------------------------------------------------------------------------------+

#![allow(dead_code)]

// -------------------------------------------------------------------------------------------------
// DEPENDENCIES
// -------------------------------------------------------------------------------------------------

// system
use std::ffi::c_void;

// local
use crate::display::{ _3dFormat,
               Info,
               InputFormat };

use crate::image::{ Image,
             ImageType,
             Rect };

use crate::vchi::MemHandle;

// -------------------------------------------------------------------------------------------------
// TYPES
// -------------------------------------------------------------------------------------------------

pub type DisplayHandle  = u32;
pub type ElementHandle  = u32;
pub type ResourceHandle = u32;
pub type UpdateHandle   = u32;

pub type CallbackFunc = extern "C" fn(handle: UpdateHandle, arg: *mut c_void);
pub type Protection   = u32;

// -------------------------------------------------------------------------------------------------
// ENUMS
// -------------------------------------------------------------------------------------------------

#[repr(C)]
pub enum FlagsAlpha {
    // bottom 2 bits sets the alpha mode
    FROM_SOURCE       = 0,
    FIXED_ALL_PIXELS  = 1,
    FIXED_NON_ZERO    = 2,
    FIXED_EXCEED_0X07 = 3,

    PREMULT = 1 << 16,
    MIX     = 1 << 17
}

#[repr(C)]
pub enum FlagsClamp {
    NONE             = 0,
    LUMA_TRANSPARENT = 1,
    TRANSPARENT      = 2,
    REPLACE          = 3
}

#[repr(C)]
pub enum FlagsKeymask {
    OVERRIDE = 1,
    SMOOTH   = 1 << 1,
    CR_INV   = 1 << 2,
    CB_INV   = 1 << 3,
    YY_INV   = 1 << 4
}

#[repr(C)]
pub enum Status {
    SUCCESS = 0,
    INVALID = -1
}

#[repr(C)]
pub enum Transform {
    NO_ROTATE  = 0,
    ROTATE_90  = 1,
    ROTATE_180 = 2,
    ROTATE_270 = 3,

    FLIP_HRIZ = 1 << 16,
    FLIP_VERT = 1 << 17,

    // extra flags for controlling snapshot behaviour
    SNAPSHOT_NO_YUV        = 1 << 24,
    SNAPSHOT_NO_RGB        = 1 << 25,
    SNAPSHOT_FILL          = 1 << 26,
    SNAPSHOT_SWAP_RED_BLUE = 1 << 27,
    SNAPSHOT_PACK          = 1 << 28
}

// -------------------------------------------------------------------------------------------------
// STRUCTS
// -------------------------------------------------------------------------------------------------

#[repr(C)]
pub struct Alpha {
    pub flags:   FlagsAlpha,
    pub opacity: u32,
    pub mask:    Image
}

#[repr(C)]
pub struct Clamp {
    pub mode:          FlagsClamp,
    pub key_mask:      FlagsKeymask,
    pub key_value:     *mut c_void,
    pub replace_value: u32
}

#[repr(C)]
pub struct ClampKeysRGB {
    pub red_upper:   u8,
    pub red_lower:   u8,
    pub blue_upper:  u8,
    pub blue_lower:  u8,
    pub green_upper: u8,
    pub green_lower: u8
}

#[repr(C)]
pub struct ClampKeysYUV {
    pub yy_upper: u8,
    pub yy_lower: u8,
    pub cr_upper: u8,
    pub cr_lower: u8,
    pub cb_upper: u8,
    pub cb_lower: u8
}

#[repr(C)]
pub struct DisplayFuncs {
    pub get_hvs_config: extern "C" fn(instance: *mut c_void, pchan: *mut u32,
                                      poptions: *mut u32, info: *mut Info,
                                      bg_color: *mut u32, test_mode: *mut u32),

    pub get_gamma_params: extern "C" fn(instance: *mut c_void,
                                        gain: [i32; 3], offset: [i32; 3],
                                        gamma: [i32; 3]),

    pub get_oled_params: extern "C" fn(instance: *mut c_void, poffsets: *mut u32,
                                       coeffs: [u32; 3]) -> i32,

    pub get_dither: extern "C" fn(instance: *mut c_void, dither_depth: *mut u32,
                                  dither_type: *mut u32) -> i32,

    pub get_info: extern "C" fn(instance: *mut c_void, info: *mut Modeinfo) -> i32,

    pub open: extern "C" fn(instance: *mut c_void) -> i32,

    pub close: extern "C" fn(instance: *mut c_void) -> i32,

    // todo: fifo_reg should be volatile
    pub dlist_updated: extern "C" fn(instance: *mut c_void, fifo_reg: *mut u32),

    pub eof_callback: extern "C" fn(instance: *mut c_void),

    pub get_input_format: extern "C" fn(instance: *mut c_void) -> InputFormat,

    pub suspend_resume: extern "C" fn(instance: *mut c_void, up: i32) -> i32,

    pub get_3d_format: extern "C" fn(instance: *mut c_void) -> _3dFormat
}

#[repr(C)]
pub struct Modeinfo {
    pub width:        i32,
    pub height:       i32,
    pub transform:    Transform,
    pub input_format: InputFormat
}

#[repr(C)]
pub struct VCAlpha {
    pub flags:   FlagsAlpha,
    pub opacity: u32,
    pub mask:    ResourceHandle
}

#[repr(C)]
pub struct Window {
    pub element: ElementHandle,
    pub width:   i32,
    pub height:  i32
}

// -------------------------------------------------------------------------------------------------
// CONSTANTS
// -------------------------------------------------------------------------------------------------

pub const DISPMANX_NO_HANDLE:       u32 = 0;
pub const DISPMANX_PROTECTION_MAX:  u32 = 0x0f;
pub const DISPMANX_PROTECTION_NONE: u32 = 0;
pub const DISPMANX_PROTECTION_HDCP: u32 = 11; // derived from the WM DRM levels, 101-300

pub const DISPMANX_ID_MAIN_LCD:    u32 = 0;
pub const DISPMANX_ID_AUX_LCD:     u32 = 1;
pub const DISPMANX_ID_HDMI:        u32 = 2;
pub const DISPMANX_ID_SDTV:        u32 = 3;
pub const DISPMANX_ID_FORCE_LCD:   u32 = 4;
pub const DISPMANX_ID_FORCE_TV:    u32 = 5;
pub const DISPMANX_ID_FORCE_OTHER: u32 = 6; // non-default display

// -------------------------------------------------------------------------------------------------
// FUNCTIONS
// -------------------------------------------------------------------------------------------------

pub fn display_close(display: DisplayHandle) -> bool {
    unsafe {
        ffi::vc_dispmanx_display_close(display) > 0
    }
}

pub fn display_get_info(display: DisplayHandle, modeinfo: *mut Modeinfo) -> bool {
    unsafe {
        ffi::vc_dispmanx_display_get_info(display, modeinfo) > 0
    }
}

pub fn display_open(device: u32) -> DisplayHandle {
    unsafe {
        ffi::vc_dispmanx_display_open(device)
    }
}

pub fn display_open_mode(device: u32, mode: u32) -> DisplayHandle {
    unsafe {
        ffi::vc_dispmanx_display_open_mode(device, mode)
    }
}

pub fn display_open_offscreen(dest: ResourceHandle, orientation: Transform) -> DisplayHandle {
    unsafe {
        ffi::vc_dispmanx_display_open_offscreen(dest, orientation)
    }
}

pub fn display_reconfigure(display: DisplayHandle, mode: u32) -> bool {
    unsafe {
        ffi::vc_dispmanx_display_reconfigure(display, mode) > 0
    }
}

pub fn display_set_background(update: UpdateHandle, display: DisplayHandle,
                              red: u8, green: u8, blue: u8) -> bool {
    unsafe {
        ffi::vc_dispmanx_display_set_background(update, display, red, green, blue) > 0
    }
}

pub fn display_set_destination(display: DisplayHandle, dest: ResourceHandle) -> bool {
    unsafe {
        ffi::vc_dispmanx_display_set_destination(display, dest) > 0
    }
}

pub fn element_add(update: UpdateHandle, display: DisplayHandle, layer: i32,
                   dest_rect: *mut Rect, src: ResourceHandle, src_rect: *mut Rect,
                   protection: Protection, alpha: *mut VCAlpha, clamp: *mut Clamp,
                   transform: Transform) -> ElementHandle {
    unsafe {
        ffi::vc_dispmanx_element_add(update, display, layer, dest_rect, src, src_rect,
                                     protection, alpha, clamp, transform)
    }
}

pub fn element_change_attributes(update: UpdateHandle, element: ElementHandle,
                                 change_flags: u32, layer: i32, opacity: u8,
                                 dest_rect: *const Rect, src_rect: *const Rect,
                                 mask: ResourceHandle, transform: Transform) -> bool {
    unsafe {
        ffi::vc_dispmanx_element_change_attributes(update, element, change_flags, layer,
                                                   opacity, dest_rect, src_rect,
                                                   mask, transform) > 0
    }
}

pub fn element_change_layer(update: UpdateHandle, element: ElementHandle,
                            layer: i32) -> bool {
    unsafe {
        ffi::vc_dispmanx_element_change_layer(update, element, layer) > 0
    }
}

pub fn element_change_source(update: UpdateHandle, element: ElementHandle,
                             src: ResourceHandle) -> bool {
    unsafe {
        ffi::vc_dispmanx_element_change_source(update, element, src) > 0
    }
}

pub fn element_modified(update: UpdateHandle, element: ElementHandle, rect: *mut Rect) -> bool {
    unsafe {
        ffi::vc_dispmanx_element_modified(update, element, rect) > 0
    }
}

pub fn element_remove(update: UpdateHandle, element: ElementHandle) -> bool {
    unsafe {
        ffi::vc_dispmanx_element_remove(update, element) > 0
    }
}

pub fn query_image_formats(supported_formats: *mut u32) -> bool {
    unsafe {
        ffi::vc_dispmanx_query_image_formats(supported_formats) > 0
    }
}

pub fn rect_set(rect: *mut Rect, x_offset: u32, y_offset: u32, width: u32,
                height: u32) -> bool {
    unsafe {
        ffi::vc_dispmanx_rect_set(rect, x_offset, y_offset, width, height) > 0
    }
}

pub fn resource_create(type_: ImageType, width: u32, height: u32,
                       native_image_handle: *mut u32) -> ResourceHandle {
    unsafe {
        ffi::vc_dispmanx_resource_create(type_, width, height, native_image_handle)
    }
}

pub fn resource_delete(res: ResourceHandle) -> bool {
    unsafe {
        ffi::vc_dispmanx_resource_delete(res) > 0
    }
}

pub fn resource_read_data(res: ResourceHandle, rect: *const Rect, dst_address: *mut c_void,
                          dst_pitch: u32) -> bool {
    unsafe {
        ffi::vc_dispmanx_resource_read_data(res, rect, dst_address, dst_pitch) > 0
    }
}

pub fn resource_set_palette(res: ResourceHandle, src_address: *mut c_void, offset: i32,
                            size: i32) -> bool {
    unsafe {
        ffi::vc_dispmanx_resource_set_palette(res, src_address, offset, size) > 0
    }
}

pub fn resource_write_data(res: ResourceHandle, src_type: ImageType, src_pitch: i32,
                           src_address: *mut c_void, rect: *const Rect) -> bool {
    unsafe {
        ffi::vc_dispmanx_resource_write_data(res, src_type, src_pitch, src_address, rect) > 0
    }
}

pub fn resource_write_data_handle(res: ResourceHandle, src_type: ImageType, src_pitch: i32,
                                  handle: MemHandle, offset: u32,
                                  rect: *const Rect) -> bool {
    unsafe {
        ffi::vc_dispmanx_resource_write_data_handle(res, src_type, src_pitch, handle, offset,
                                                    rect) > 0
    }
}

pub fn snapshot(display: DisplayHandle, snapshot_resource: ResourceHandle,
                transform: Transform) -> bool {
    unsafe {
        ffi::vc_dispmanx_snapshot(display, snapshot_resource, transform) > 0
    }
}

pub fn stop() {
    unsafe {
        ffi::vc_dispmanx_stop()
    }
}

pub fn update_start(priority: i32) -> UpdateHandle {
    unsafe {
        ffi::vc_dispmanx_update_start(priority)
    }
}

pub fn update_submit(update: UpdateHandle, callback_func: CallbackFunc,
                     callback_arg: *mut c_void) -> bool {
    unsafe {
        ffi::vc_dispmanx_update_submit(update, callback_func, callback_arg) > 0
    }
}

pub fn update_submit_sync(update: UpdateHandle) -> bool {
    unsafe {
        ffi::vc_dispmanx_update_submit_sync(update) > 0
    }
}

pub fn vsync_callback(display: DisplayHandle, callback_func: CallbackFunc,
                      callback_arg: *mut c_void) -> bool {
    unsafe {
        ffi::vc_dispmanx_vsync_callback(display, callback_func, callback_arg) > 0
    }
}

// -------------------------------------------------------------------------------------------------
// FFI
// -------------------------------------------------------------------------------------------------

mod ffi {
    use std::ffi::c_void;

    use crate::image::{ ImageType,
                 Rect };

    use crate::vchi::MemHandle;

    use super::*;

    extern {
        // deprecated
        pub fn vc_dispman_init() -> i32;

        pub fn vc_dispmanx_display_close(display: DisplayHandle) -> i32;

        pub fn vc_dispmanx_display_get_info(display: DisplayHandle,
                                            pinfo: *mut Modeinfo) -> i32;

        pub fn vc_dispmanx_display_open(device: u32) -> DisplayHandle;

        pub fn vc_dispmanx_display_open_mode(device: u32, mode: u32) -> DisplayHandle;

        pub fn vc_dispmanx_display_open_offscreen(dest: ResourceHandle,
                                                  orientation: Transform) -> DisplayHandle;

        pub fn vc_dispmanx_display_reconfigure(display: DisplayHandle, mode: u32) -> i32;

        pub fn vc_dispmanx_display_set_background(update: UpdateHandle,
                                                  display: DisplayHandle,
                                                  red: u8, green: u8,
                                                  blue: u8) -> i32;

        pub fn vc_dispmanx_display_set_destination(display: DisplayHandle,
                                                   dest: ResourceHandle) -> i32;

        pub fn vc_dispmanx_element_add(update: UpdateHandle, display: DisplayHandle,
                                       layer: i32, dest_rect: *mut Rect,
                                       src: ResourceHandle, src_rect: *mut Rect,
                                       protection: Protection, alpha: *mut VCAlpha,
                                       clamp: *mut Clamp, transform: Transform) -> ElementHandle;

        pub fn vc_dispmanx_element_change_attributes(update: UpdateHandle,
                                                     element: ElementHandle,
                                                     change_flags: u32, layer: i32,
                                                     opacity: u8, dest_rect: *const Rect,
                                                     src_rect: *const Rect, mask: ResourceHandle,
                                                     transform: Transform) -> i32;

        pub fn vc_dispmanx_element_change_layer(update: UpdateHandle, element: ElementHandle,
                                                layer: i32) -> i32;

        pub fn vc_dispmanx_element_change_source(update: UpdateHandle, element: ElementHandle,
                                                 src: ResourceHandle) -> i32;

        pub fn vc_dispmanx_element_modified(update: UpdateHandle, element: ElementHandle,
                                            rect: *mut Rect) -> i32;

        pub fn vc_dispmanx_element_remove(update: UpdateHandle, element: ElementHandle) -> i32;

        pub fn vc_dispmanx_query_image_formats(supported_formats: *mut u32) -> i32;

        pub fn vc_dispmanx_rect_set(rect: *mut Rect, x_offset: u32, y_offset: u32,
                                    width: u32, height: u32) -> i32;

        pub fn vc_dispmanx_resource_create(type_: ImageType, width: u32, height: u32,
                                           native_image_handle: *mut u32) -> ResourceHandle;

        pub fn vc_dispmanx_resource_delete(res: ResourceHandle) -> i32;

        // deprecated
        pub fn vc_dispmanx_resource_get_image_handle(res: ResourceHandle) -> u32;

        pub fn vc_dispmanx_resource_read_data(handle: ResourceHandle, p_rect: *const Rect,
                                              dst_address: *mut c_void,
                                              dst_pitch: u32) -> i32;

        pub fn vc_dispmanx_resource_set_palette(handle: ResourceHandle,
                                                src_address: *mut c_void, offset: i32,
                                                size: i32) -> i32;

        pub fn vc_dispmanx_resource_write_data(res: ResourceHandle, src_type: ImageType,
                                               src_pitch: i32, src_address: *mut c_void,
                                               rect: *const Rect) -> i32;

        pub fn vc_dispmanx_resource_write_data_handle(res: ResourceHandle, src_type: ImageType,
                                                      src_pitch: i32, handle: MemHandle,
                                                      offset: u32,
                                                      rect: *const Rect) -> i32;

        pub fn vc_dispmanx_snapshot(display: DisplayHandle,
                                    snapshot_resource: ResourceHandle,
                                    transform: Transform) -> i32;

        pub fn vc_dispmanx_stop();

        pub fn vc_dispmanx_update_start(priority: i32) -> UpdateHandle;

        pub fn vc_dispmanx_update_submit(update: UpdateHandle, cb_func: CallbackFunc,
                                         cb_arg: *mut c_void) -> i32;

        pub fn vc_dispmanx_update_submit_sync(update: UpdateHandle) -> i32;

        pub fn vc_dispmanx_vsync_callback(display: DisplayHandle, cb_func: CallbackFunc,
                                          cb_arg: *mut c_void) -> i32;

        // call this instead of vc_dispman_init()
        //pub fn vc_vchi_dispmanx_init(VCHI_INSTANCE_T initialise_instance, VCHI_CONNECTION_T **connections, u32 num_connections );
    }
}
