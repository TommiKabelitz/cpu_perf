use std::ffi::{CString, NulError};
use std::ptr;

use c_interface::event_types::{XEventMask, XEventType};
use c_interface::xgc_values::XgcValues;
use c_interface::{
    Display, GraphicsContext, Screen, Window, XCloseDisplay, XCreateGC, XCreateImage,
    XCreateSimpleWindow, XDefaultDepth, XDefaultScreen, XDefaultVisual, XDestroyWindow, XEvent,
    XImage, XMapWindow, XNextEvent, XOpenDisplay, XPutImage, XRootWindow, XSelectInput, XStoreName,
};

mod c_interface;

pub struct X11Window {
    display: *mut Display,
    screen: Screen,
    root_window: Window,
    window: Window,
    gc: *mut GraphicsContext,
    image: *mut XImage,
    width: u32,
    height: u32,
}

impl X11Window {
    pub fn new<'a>(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border_width: u32,
        border: u64,
        background: u64,
        data_buffer: &'a Vec<u32>,
    ) -> Option<Self> {
        let display = unsafe { XOpenDisplay(ptr::null()) };
        if display.is_null() {
            return None;
        }
        let screen = unsafe { XDefaultScreen(display) };
        let root_window = unsafe { XRootWindow(display, screen) };
        let window = unsafe {
            XCreateSimpleWindow(
                display,
                root_window,
                x,
                y,
                width,
                height,
                border_width,
                border,
                background,
            )
        };

        let xgc_values = XgcValues::default();
        let gc = unsafe { XCreateGC(display, window, 0, &xgc_values as *const XgcValues) };

        let visual = unsafe { XDefaultVisual(display, screen) };
        let depth = unsafe { XDefaultDepth(display, screen) } as u32;

        let image = unsafe {
            XCreateImage(
                display,
                visual,
                depth,
                2, // ZPixmap
                0,
                data_buffer.as_ptr() as *const i8,
                width,
                height,
                32,
                0,
            )
        };

        let event_mask =
            XEventMask::EXPOSURE | XEventMask::KEY_PRESS | XEventMask::STRUCTURE_NOTIFY;
        unsafe {
            XSelectInput(display, window, event_mask.bits() as i64);
        }

        Some(Self {
            display,
            screen,
            root_window,
            window,
            gc,
            image,
            width,
            height,
        })
    }

    pub fn set_title(&self, title: &str) -> Result<(), NulError> {
        let title = CString::new(title)?;
        unsafe { XStoreName(self.display, self.window, title.as_ptr()) };
        Ok(())
    }

    pub fn show(&self) {
        unsafe {
            XMapWindow(self.display, self.window);
        }
    }

    pub fn update_window(&self) {
        unsafe {
            XPutImage(
                self.display,
                self.window,
                self.gc,
                self.image,
                0,
                0,
                0,
                0,
                self.width,
                self.height,
            )
        };
    }

    pub fn wait_map_notify(&self) {
        unsafe {
            let mut event: XEvent = std::mem::zeroed();
            loop {
                XNextEvent(self.display, &mut event);
                if event.type_ == XEventType::MapNotify as i32 {
                    break;
                }
            }
        }
    }
}

impl Drop for X11Window {
    fn drop(&mut self) {
        if self.display.is_null() {
            return;
        }
        unsafe {
            XDestroyWindow(self.display, self.window);
            XCloseDisplay(self.display);
        }
    }
}
