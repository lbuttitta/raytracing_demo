#[derive(Copy, Clone, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl Color {
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0 };

    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0 };

    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0 };

    pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0 };

    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0 };

    pub const MAGENTA: Color = Color { r: 1.0, g: 0.0, b: 1.0 };

    pub const CYAN: Color = Color { r: 0.0, g: 1.0, b: 1.0 };

    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0 };
}

impl From<Color> for [u8; 4] {
    fn from(color: Color) -> Self {
        [
            (color.r * 256.0) as u8,
            (color.g * 256.0) as u8,
            (color.b * 256.0) as u8,
            255
        ]
    }
}
