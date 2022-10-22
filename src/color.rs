#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };

    pub const RED: Color = Color { r: 255, g: 0, b: 0 };

    pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };

    pub const YELLOW: Color = Color { r: 255, g: 255, b: 0 };

    pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };

    pub const MAGENTA: Color = Color { r: 255, g: 0, b: 255 };

    pub const CYAN: Color = Color { r: 0, g: 255, b: 255 };

    pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };
}

impl From<Color> for [u8; 4] {
    fn from(color: Color) -> Self {
        [ color.r, color.g, color.b, 255 ]
    }
}
