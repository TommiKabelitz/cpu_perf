use std::os::raw::c_int;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)] // So values line up with X11's event type numbers
pub enum XEventType {
    KeyPress = 2,
    KeyRelease = 3,
    ButtonPress = 4,
    ButtonRelease = 5,
    MotionNotify = 6,
    EnterNotify = 7,
    LeaveNotify = 8,
    FocusIn = 9,
    FocusOut = 10,
    KeymapNotify = 11,
    Expose = 12,
    GraphicsExpose = 13,
    NoExpose = 14,
    VisibilityNotify = 15,
    CreateNotify = 16,
    DestroyNotify = 17,
    UnmapNotify = 18,
    MapNotify = 19,
    MapRequest = 20,
    ReparentNotify = 21,
    ConfigureNotify = 22,
    ConfigureRequest = 23,
    GravityNotify = 24,
    ResizeRequest = 25,
    CirculateNotify = 26,
    CirculateRequest = 27,
    PropertyNotify = 28,
    SelectionClear = 29,
    SelectionRequest = 30,
    SelectionNotify = 31,
    ColormapNotify = 32,
    ClientMessage = 33,
    MappingNotify = 34,
    GenericEvent = 35,
}

impl XEventType {
    /// Convert a raw XEvent.type_ into an enum, if known.
    pub fn from_raw(raw: c_int) -> Option<Self> {
        use XEventType::*;
        Some(match raw {
            2 => KeyPress,
            3 => KeyRelease,
            4 => ButtonPress,
            5 => ButtonRelease,
            6 => MotionNotify,
            7 => EnterNotify,
            8 => LeaveNotify,
            9 => FocusIn,
            10 => FocusOut,
            11 => KeymapNotify,
            12 => Expose,
            13 => GraphicsExpose,
            14 => NoExpose,
            15 => VisibilityNotify,
            16 => CreateNotify,
            17 => DestroyNotify,
            18 => UnmapNotify,
            19 => MapNotify,
            20 => MapRequest,
            21 => ReparentNotify,
            22 => ConfigureNotify,
            23 => ConfigureRequest,
            24 => GravityNotify,
            25 => ResizeRequest,
            26 => CirculateNotify,
            27 => CirculateRequest,
            28 => PropertyNotify,
            29 => SelectionClear,
            30 => SelectionRequest,
            31 => SelectionNotify,
            32 => ColormapNotify,
            33 => ClientMessage,
            34 => MappingNotify,
            35 => GenericEvent,
            _ => return None,
        })
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct XEventMask(u64);
#[allow(dead_code)]
impl XEventMask {
    pub const KEY_PRESS: Self = Self(1 << 0);
    pub const KEY_RELEASE: Self = Self(1 << 1);
    pub const BUTTON_PRESS: Self = Self(1 << 2);
    pub const BUTTON_RELEASE: Self = Self(1 << 3);
    pub const ENTER_WINDOW: Self = Self(1 << 4);
    pub const LEAVE_WINDOW: Self = Self(1 << 5);
    pub const POINTER_MOTION: Self = Self(1 << 6);
    pub const POINTER_MOTION_HINT: Self = Self(1 << 7);
    pub const BUTTON1_MOTION: Self = Self(1 << 8);
    pub const BUTTON2_MOTION: Self = Self(1 << 9);
    pub const BUTTON3_MOTION: Self = Self(1 << 10);
    pub const BUTTON4_MOTION: Self = Self(1 << 11);
    pub const BUTTON5_MOTION: Self = Self(1 << 12);
    pub const BUTTON_MOTION: Self = Self(1 << 13);
    pub const KEYMAP_STATE: Self = Self(1 << 14);
    pub const EXPOSURE: Self = Self(1 << 15);
    pub const VISIBILITY_CHANGE: Self = Self(1 << 16);
    pub const STRUCTURE_NOTIFY: Self = Self(1 << 17);
    pub const RESIZE_REDIRECT: Self = Self(1 << 18);
    pub const SUBSTRUCTURE_NOTIFY: Self = Self(1 << 19);
    pub const SUBSTRUCTURE_REDIRECT: Self = Self(1 << 20);
    pub const FOCUS_CHANGE: Self = Self(1 << 21);
    pub const PROPERTY_CHANGE: Self = Self(1 << 22);
    pub const COLORMAP_CHANGE: Self = Self(1 << 23);
    pub const OWNER_GRAB_BUTTON: Self = Self(1 << 24);

    pub fn bits(self) -> u64 {
        self.0
    }
}

impl From<u64> for XEventMask {
    fn from(bits: u64) -> Self {
        Self(bits)
    }
}

impl From<XEventMask> for u64 {
    fn from(mask: XEventMask) -> Self {
        mask.bits()
    }
}

impl std::ops::BitOr for XEventMask {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for XEventMask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl std::ops::BitAnd for XEventMask {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for XEventMask {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::Not for XEventMask {
    type Output = Self;

    fn not(self) -> Self {
        Self(!self.0)
    }
}
