/// Color struct.
///
/// Represents a color with RGB values from 0 to 255.
impl Color {

    /// Constructor.
    ///
    /// The Red, Green and Blue values must be between 0 and 255.
    pub fn from(r: u8, g: u8, b: u8) -> Color {
        Color {
            r,
            g,
            b,
        }
    }

    /// Get a hexadecimal representation of the color,
    /// such as `FFFFFF` for white and `FF0000` for red.
    pub fn as_hex(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}
