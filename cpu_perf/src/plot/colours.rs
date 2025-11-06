#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Colour {
    BLACK = 0xff000000,
    WHITE = 0xffffffff,
    GREY = 0xff2e2e2e,
    GREEN = 0xff00ff00,
    BLUE = 0xff0000ff,
}
