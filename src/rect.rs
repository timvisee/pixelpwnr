/// Rectangle struct.
#[derive(Copy, Clone)]
pub struct Rect {
    // TODO: Make these properties private
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn from(x: u32, y: u32, w: u32, h: u32) -> Rect {
        Rect {
            x,
            y,
            w,
            h,
        }
    }
}
