/// Rectangle struct.
#[derive(Copy, Clone)]
pub struct Rect {
    // TODO: Make these properties private
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

impl Rect {
    pub fn from(x: u16, y: u16, w: u16, h: u16) -> Rect {
        Rect { x, y, w, h }
    }
}
