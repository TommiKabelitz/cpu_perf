use std::os::raw::{c_char, c_int, c_ulong};

use super::Window;

#[repr(C)]
pub struct XgcValues {
    pub function: c_int,
    pub plane_mask: c_ulong,
    pub foreground: c_ulong,
    pub background: c_ulong,
    pub line_width: c_int,
    pub line_style: c_int,
    pub cap_style: c_int,
    pub join_style: c_int,
    pub fill_style: c_int,
    pub fill_rule: c_int,
    pub arc_mode: c_int,
    pub font: c_ulong,
    pub subwindow_mode: c_int,
    pub graphics_exposures: c_int,
    pub clip_x_origin: c_int,
    pub clip_y_origin: c_int,
    pub clip_mask: Window, // drawable mask
    pub dash_offset: c_int,
    pub dashes: *const c_char,
}

impl Default for XgcValues {
    fn default() -> Self {
        Self {
            function: 0,
            plane_mask: 0,
            foreground: 0,
            background: 0,
            line_width: 0,
            line_style: 0,
            cap_style: 0,
            join_style: 0,
            fill_style: 0,
            fill_rule: 0,
            arc_mode: 0,
            font: 0,
            subwindow_mode: 0,
            graphics_exposures: 0,
            clip_x_origin: 0,
            clip_y_origin: 0,
            clip_mask: 0,
            dash_offset: 0,
            dashes: std::ptr::null(),
        }
    }
}
