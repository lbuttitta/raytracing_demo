use std::iter::Sum;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs
        }
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, other: f64) {
        self.r /= other;
        self.g /= other;
        self.b /= other;
    }
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

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b
        }
    }
}

impl MulAssign<Color> for Color {
    fn mul_assign(&mut self, other: Color) {
        self.r *= other.r;
        self.g *= other.g;
        self.b *= other.b;
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs
        }
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, other: f64) {
        self.r *= other;
        self.g *= other;
        self.b *= other;
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b
        }
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, other: Self) {
        self.r -= other.r;
        self.g -= other.g;
        self.b -= other.b;
    }
}

impl Sum for Color {
    fn sum<I>(iter: I) -> Self where I: Iterator<Item = Self> {
        let mut total = Color::BLACK;
        for color in iter {
            total += color;
        }
        total
    }
}
