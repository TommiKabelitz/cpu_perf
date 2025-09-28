use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong};

use xgc_values::XgcValues;

pub mod event_types;
pub mod xgc_values;

#[repr(C)]
pub struct Display {
    _private: [u8; 0],
}
#[repr(C)]
pub struct Visual {
    _private: [u8; 0],
}
#[repr(C)]
pub struct XSetWindowAttributes {
    _private: [u8; 0],
}

#[repr(C)]
pub struct XImage {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GraphicsContext {
    _private: [u8; 0],
}

pub type Window = c_ulong;
pub type Screen = c_int;
pub type Atom = c_ulong;
pub type Bool = c_int;
pub type Status = c_int;

#[repr(C)]
pub struct XEvent {
    pub type_: c_int,
    pub pad: [c_long; 24], // oversize placeholder (XEvent is a union)
}

#[link(name = "X11")]
unsafe extern "C" {
    /// `XOpenDisplay` - connect to X server (see `XCloseDisplay` for disconnect)
    ///
    /// ## C Syntax
    /// ```
    /// Display *XOpenDisplay(char *display_name);
    /// ```
    /// ## C Arguments
    /// `display_name`: Specifies the hardware display name, which determines the display and communications domain to be used. On a POSIX-conformant system, if the display_name is `NULL`, it defaults to the value of the `DISPLAY` environment variable.
    ///
    /// ## Description
    /// The `XOpenDisplay` function returns a `Display` structure that serves as the connection to the X server and that contains all the information about that X server. `XOpenDisplay` connects your application to the X server through TCP or DECnet communications protocols, or through some local inter-process communication protocol. If the hostname is a host machine name and a single colon (`:`) separates the hostname and display number, `XOpenDisplay` connects using TCP streams. If the hostname is not specified, Xlib uses whatever it believes is the fastest transport. If the hostname is a host machine name and a double colon (`::`) separates the hostname and display number, `XOpenDisplay` connects using DECnet. A single X server can support any or all of these transport mechanisms simultaneously. A particular Xlib implementation can support many more of these transport mechanisms.
    ///
    /// If successful, `XOpenDisplay` returns a pointer to a `Display` structure, which is defined in `<X11/Xlib.h>`. If `XOpenDisplay` does not succeed, it returns `NULL`. After a successful call to `XOpenDisplay`, all of the screens in the display can be used by the client. The screen number specified in the `display_name` argument is returned by the `DefaultScreen` macro (or the `XDefaultScreen` function). You can access elements of the `Display` and `Screen` structures only by using the information macros or functions. For information about using macros and functions to obtain information from the `Display` structure, see section 2.2.1.
    ///
    /// https://www.x.org/archive/X11R7.5/doc/man/man3/XOpenDisplay.3.html
    pub fn XOpenDisplay(display_name: *const c_char) -> *mut Display;

    /// The `DefaultScreen` macro returns the default screen number referenced in the `XOpenDisplay` routine.
    ///
    /// ## C Syntax
    /// ```
    /// int DefaultScreen(Display *display);
    /// ```
    ///
    /// ## C Arguments
    ///
    /// `display` - Specifies the connection to the X server.
    ///
    /// https://www.x.org/archive/X11R7.5/doc/man/man3/DefaultRootWindow.3.html
    pub fn XDefaultScreen(display: *mut Display) -> Screen;

    /// The `RootWindow` macro returns the root window.
    ///
    /// ## C Syntax
    /// ```
    /// Window RootWindow(Display *display, int screen_number);
    /// ```
    ///
    /// ## C Arguments
    ///
    /// `display` - Specifies the connection to the X server.
    ///
    /// `screen_number` - Specifies the appropriate screen number on the host server.
    ///
    /// https://www.x.org/archive/X11R7.5/doc/man/man3/DefaultRootWindow.3.html
    pub fn XRootWindow(display: *mut Display, screen_number: Screen) -> Window;

    /// `XCreateWindow`, `XCreateSimpleWindow`, `XSetWindowAttributes` - create windows and window attributes struct
    ///
    /// ## C Syntax
    /// ```
    /// Window XCreateSimpleWindow(Display *display, Window parent, int x, int y, unsigned int width, unsigned int height, unsigned int border_width, unsigned long border, unsigned long background);
    /// ```
    ///
    /// ## C Arguments
    ///
    /// `background` - Specifies the background pixel value of the window.
    ///
    /// `border` - Specifies the border pixel value of the window.
    ///
    /// `border_width` - Specifies the width of the created window's border in pixels.
    ///
    /// `display` - Specifies the connection to the X server.
    ///
    /// `parent` - Specifies the parent window.
    ///
    /// `width`, `height` - Specify the width and height, which are the created window's inside dimensions . borders and are relative to the inside of the parent window's borders
    ///
    /// `x`, `y` - Specify the x and y coordinates, which are the top-left outside corner of the window's .
    ///
    /// ## Description
    ///
    /// The `XCreateSimpleWindow` function creates an unmapped `InputOutput` subwindow for a specified parent window, returns the window ID of the created window, and causes the X server to generate a `CreateNotify` event. The created window is placed on top in the stacking order with respect to siblings. Any part of the window that extends outside its parent window is clipped. The border_width for an `InputOnly` window must be zero, or a `BadMatch` error results. `XCreateSimpleWindow` inherits its depth, class, and visual from its parent. All other window attributes, except background and border, have their default values.
    ///
    /// The coordinate system has the X axis horizontal and the Y axis vertical with the origin [0, 0] at the upper-left corner. Coordinates are integral, in terms of pixels, and coincide with pixel centers. Each window and pixmap has its own coordinate system. For a window, the origin is inside the border at the inside, upper-left corner.
    ///
    /// The `border_width` for an `InputOnly` window must be zero, or a `BadMatch` error results. For class `InputOutput`, the visual type and depth must be a combination supported for the screen, or a `BadMatch` error results. The depth need not be the same as the parent, but the parent must not be a window of class InputOnly, or a `BadMatch` error results. For an `InputOnly` window, the depth must be zero, and the visual must be one supported by the screen. If either condition is not met, a `BadMatch` error results. The parent window, however, may have any depth and class. If you specify any invalid window attribute for a window, a `BadMatch` error results.
    ///
    /// The created window is not yet displayed (mapped) on the user's display. To display the window, call `XMapWindow`. The new window initially uses the same cursor as its parent. A new cursor can be defined for the new window by calling `XDefineCursor`. The window will not be visible on the screen unless it and all of its ancestors are mapped and it is not obscured by any of its ancestors.
    ///
    /// `XCreateSimpleWindow` can generate `BadAlloc`, `BadMatch`, `BadValue`, and `BadWindow` errors.
    ///
    /// https://www.x.org/archive/X11R7.5/doc/man/man3/XCreateSimpleWindow.3.html
    pub fn XCreateSimpleWindow(
        display: *mut Display,
        parent: Window,
        x: c_int,
        y: c_int,
        width: c_uint,
        height: c_uint,
        border_width: c_uint,
        border: c_ulong,
        background: c_ulong,
    ) -> Window;

    /// `XMapWindow` - map windows
    ///
    /// ## C Syntax
    /// ```
    /// int XMapWindow(Display *display, Window w);
    /// ```
    /// ## C Arguments
    ///
    /// `display` - Specifies the connection to the X server.
    /// `w` - Specifies the window.
    ///
    /// ## Description
    /// The `XMapWindow` function maps the window and all of its subwindows that have had map requests. Mapping a window that has an unmapped ancestor does not display the window but marks it as eligible for display when the ancestor becomes mapped. Such a window is called unviewable. When all its ancestors are mapped, the window becomes viewable and will be visible on the screen if it is not obscured by another window. This function has no effect if the window is already mapped.
    ///
    /// If the override-redirect of the window is `False` and if some other client has selected `SubstructureRedirectMask `on the parent window, then the X server generates a `MapRequest` event, and the `XMapWindow` function does not map the window. Otherwise, the window is mapped, and the X server generates a `MapNotify` event.
    ///
    /// If the window becomes viewable and no earlier contents for it are remembered, the X server tiles the window with its background. If the window's background is undefined, the existing screen contents are not altered, and the X server generates zero or more `Expose` events. If backing-store was maintained while the window was unmapped, no `Expose` events are generated. If `backing-store` will now be maintained, a full-window exposure is always generated. Otherwise, only visible regions may be reported. Similar tiling and exposure take place for any newly viewable inferiors.
    ///
    /// If the window is an `InputOutput` window, `XMapWindow` generates `Expose` events on each `InputOutput` window that it causes to be displayed. If the client maps and paints the window and if the client begins processing events, the window is painted twice. To avoid this, first ask for `Expose` events and then map the window, so the client processes input events as usual. The event list will include `Expose` for each window that has appeared on the screen. The client's normal response to an `Expose` event should be to repaint the window. This method usually leads to simpler programs and to proper interaction with window managers.
    ///
    /// `XMapWindow` can generate a `BadWindow` error.
    ///
    /// ## Diagnostics
    ///
    /// `BadWindow` - A value for a Window argument does not name a defined Window.
    ///
    /// https://www.x.org/archive/X11R7.5/doc/man/man3/XMapWindow.3.html
    pub fn XMapWindow(display: *mut Display, w: Window) -> c_int;

    /// `XStoreName` - set or read a window's `WM_NAME` property
    ///
    /// ## C Syntax
    /// ```
    /// int XStoreName(Display *display, Window w, char *window_name);
    /// ```
    ///
    /// ## C Arguments
    ///
    /// `display` - Specifies the connection to the X server.
    ///
    /// `w` - Specifies the window.
    ///
    /// `window_name` - Specifies the window name, which should be a null-terminated string.
    ///
    /// ## Description
    ///
    ///  The `XStoreName` function assigns the name passed to `window_name` to the specified window. A window manager can display the window name in some prominent place, such as the title bar, to allow users to identify windows easily. Some window managers may display a window's name in the window's icon, although they are encouraged to use the window's icon name if one is provided by the application. If the string is not in the Host Portable Character Encoding, the result is implementation-dependent.
    ///
    /// ## Diagnostics
    ///
    /// `XStoreName` can generate `BadAlloc` and `BadWindow` errors.
    ///
    /// https://www.x.org/archive/X11R7.5/doc/man/man3/XStoreName.3.html
    pub fn XStoreName(display: *mut Display, w: Window, window_name: *const c_char) -> c_int;

    /// `XSelectInput` - select input events
    ///
    /// ## C Syntax
    /// ```
    /// int XSelectInput(Display *display, Window w, long
    ///     event_mask);
    /// ```
    /// ## C Arguments
    ///
    /// `display` - Specifies the connection to the X server.
    ///
    /// `event_mask` - Specifies the event mask.
    ///
    /// `w` - Specifies the window whose events you are interested in.
    ///
    /// ## Description
    /// The `XSelectInput` function requests that the X server report the events associated with the specified event mask. Initially, X will not report any of these events. Events are reported relative to a window. If a window is not interested in a device event, it usually propagates to the closest ancestor that is interested, unless the `do_not_propagate` mask prohibits it.
    ///
    /// Setting the `event-mask` attribute of a window overrides any previous call for the same window but not for other clients. Multiple clients can select for the same events on the same window with the following restrictions:
    ///
    /// - Multiple clients can select events on the same window because their event masks are disjoint. When the X server generates an event, it reports it to all interested clients.
    /// - Only one client at a time can select CirculateRequest, ConfigureRequest, or MapRequest events, which are associated with the event mask SubstructureRedirectMask.
    /// - Only one client at a time can select a ResizeRequest event, which is associated with the event mask ResizeRedirectMask.
    /// - Only one client at a time can select a ButtonPress event, which is associated with the event mask ButtonPressMask.
    ///
    /// The server reports the event to all interested clients.
    ///
    /// Diagnostics
    ///
    /// `XSelectInput` can generate a `BadWindow` error.
    ///
    /// `BadWindow` - A value for a `Window` argument does not name a defined `Window`.
    ///
    /// https://www.x.org/archive/X11R7.5/doc/man/man3/XSelectInput.3.html
    pub fn XSelectInput(display: *mut Display, w: Window, event_mask: c_long) -> c_int;

    /// `XNextEvent` - select events by type
    ///
    /// ## C Syntax
    /// ```
    /// int XNextEvent(Display *display, XEvent *event_return);
    /// ```
    /// ## C Attributes
    ///
    /// `display` - Specifies the connection to the X server.
    /// `event_return` - Returns the matched event's associated structure.
    ///
    /// ## Description
    ///
    /// The `XNextEvent` function copies the first event from the event queue into the specified `XEvent` structure and then removes it from the queue. If the event queue is empty, `XNextEvent` flushes the output buffer and blocks until an event is received.
    ///
    /// https://www.x.org/archive/X11R7.5/doc/man/man3/XNextEvent.3.html
    pub fn XNextEvent(display: *mut Display, event_return: *mut XEvent) -> c_int;

    /// XDestroyWindow - destroy windows
    ///
    /// ## C Syntax
    /// ```
    /// int XDestroyWindow(Display *display, Window w);
    /// ```
    /// ## C Attributes
    ///
    /// `display` - Specifies the connection to the X server.
    /// `w` - Specifies the window.
    ///
    /// ## Description
    ///
    /// The `XDestroyWindow` function destroys the specified window as well as all of its subwindows and causes the X server to generate a `DestroyNotify` event for each window. The window should never be referenced again. If the window specified by the w argument is mapped, it is unmapped automatically. The ordering of the `DestroyNotify` events is such that for any given window being destroyed, `DestroyNotify` is generated on any inferiors of the window before being generated on the window itself. The ordering among siblings and across subhierarchies is not otherwise constrained. If the window you specified is a root window, no windows are destroyed. Destroying a mapped window will generate `Expose` events on other windows that were obscured by the window being destroyed.
    ///
    /// ## Diagnostics
    ///
    /// `XDestroyWindow` can generate a `BadWindow` error.
    ///
    /// `BadWindow` - A value for a `Window` argument does not name a defined `Window`.
    ///
    /// https://www.x.org/archive/X11R7.5/doc/man/man3/XDestroyWindow.3.html
    pub fn XDestroyWindow(display: *mut Display, w: Window) -> c_int;

    pub fn XCreateGC(
        display: *mut Display,
        drawable: Window,
        value_mask: c_ulong,
        values: *const XgcValues,
    ) -> *mut GraphicsContext;

    pub fn XFreeGC(display: *mut Display, gc: *mut GraphicsContext) -> c_int;

    pub fn XCreateImage(
        display: *mut Display,
        visual: *mut Visual,
        depth: c_uint,
        format: c_int,
        offset: c_int,
        data: *const c_char,
        width: c_uint,
        height: c_uint,
        bitmap_pad: c_int,
        bytes_per_line: c_int,
    ) -> *mut XImage;

    ///    `XPutImage` - transfer images
    ///
    ///    ## C Syntax
    ///
    ///    ```
    ///     int XPutImage(Display *display, Drawable d, GC gc, XImage *image, int src_x, int src_y, int dest_x, int dest_y, unsigned int width, unsigned int height);
    ///    ```
    ///
    ///    ## C Attributes
    ///
    ///    `d` - Specifies the drawable.
    ///
    ///    `dest_x` , `dest_y` - Specify the `x` and `y` coordinates, which are relative to the origin of the drawable .
    ///
    ///    `display` - Specifies the connection to the X server.
    ///
    ///    `gc` - Specifies the GC.
    ///
    ///    `image` - Specifies the image you want combined with the rectangle.
    ///
    ///    `src_x` - Specifies the offset in X from the left edge of the image defined by the XImage structure.
    ///
    ///    `src_y` - Specifies the offset in Y from the top edge of the image defined by the XImage structure.
    ///
    ///    `width` , `height` - Specify the width and height of the subimage, which define the dimensions of the rectangle. and define the upper-left corner of the rectangle
    ///
    ///    ## Description
    ///
    ///    The `XPutImage` function combines an image with a rectangle of the specified drawable. The section of the image defined by the `src_x`, `src_y`, `width`, and `height` arguments is drawn on the specified part of the drawable. If `XYBitmap` format is used, the depth of the image must be one, or a `BadMatch` error results. The foreground pixel in the `GC` defines the source for the one bits in the image, and the background pixel defines the source for the zero bits. For `XYPixmap` and `ZPixmap`, the depth of the image must match the depth of the drawable, or a `BadMatch` error results.
    ///
    ///    If the characteristics of the image (for example, `byte_order` and `bitmap_unit`) differ from what the server requires, `XPutImage` automatically makes the appropriate conversions.
    ///
    ///    This function uses these `GC` components: `function`, `plane-mask`, `subwindow-mode`, `clip-x-origin`, `clip-y-origin`, and `clip-mask`. It also uses these `GC` mode-dependent components: foreground and background.
    ///
    ///    `XPutImage` can generate `BadDrawable`, `BadGC`, `BadMatch`, and `BadValue` errors.
    ///
    ///    ## Diagnostics
    ///
    ///    `BadDrawable` - A value for a `Drawable` argument does not name a defined `Window` or `Pixmap`.
    ///    `BadGC` - A value for a `GContext` argument does not name a defined `GContext`.
    ///    `BadMatch` - An `InputOnly` window is used as a `Drawable`.
    ///    `BadMatch` - Some argument or pair of arguments has the correct type and range but fails to match in some other way required by the request.
    ///    `BadValue` - Some numeric value falls outside the range of values accepted by the request. Unless a specific range is specified for an argument, the full range defined by the argument's type is accepted. Any argument defined as a set of alternatives can generate this error.
    pub fn XPutImage(
        display: *mut Display,
        window: Window,
        gc: *mut GraphicsContext,
        image: *mut XImage,
        src_x: c_int,
        src_y: c_int,
        dest_x: c_int,
        dest_y: c_int,
        width: c_uint,
        height: c_uint,
    ) -> c_int;

    pub fn XDefaultVisual(display: *mut Display, screen_number: c_int) -> *mut Visual;

    pub fn XDefaultDepth(display: *mut Display, screen_number: c_int) -> c_int;

    /// #`XCloseDisplay` - disconnect from X Server
    ///
    /// ## C Syntax
    /// ```
    /// int XCloseDisplay(Display *display);
    /// ```
    /// ## C Arguments
    /// `display`: Specifies the connection to the X server.
    ///
    /// ## Description
    ///
    /// The `XCloseDisplay` function closes the connection to the X server for the display specified in the `Display` structure and destroys all windows, resource IDs (`Window`, `Font`, `Pixmap`, `Colormap`, `Cursor`, and `GContext`), or other resources that the client has created on this display, unless the close-down mode of the resource has been changed (see `XSetCloseDownMode`). Therefore, these windows, resource IDs, and other resources should never be referenced again or an error will be generated. Before exiting, you should call `XCloseDisplay` explicitly so that any pending errors are reported as `XCloseDisplay` performs a final XSync operation.
    ///
    /// ## Diagnostics
    ///
    /// `XCloseDisplay` can generate a `BadGC` error.
    ///
    /// https://www.x.org/archive/X11R7.5/doc/man/man3/XOpenDisplay.3.html
    pub fn XCloseDisplay(display: *mut Display) -> c_int;
}
