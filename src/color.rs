/// A terminal color.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    /// Black.
    Black,
    /// Red.
    Red,
    /// Green.
    Green,
    /// Yellow.
    Yellow,
    /// Blue.
    Blue,
    /// Megenta.
    Magenta,
    /// Cyan.
    Cyan,
    /// White.
    White,
    /// High-intensity black.
    LightBlack,
    /// High-intensity red.
    LightRed,
    /// High-intensity green.
    LightGreen,
    /// High-intensity yellow.
    LightYellow,
    /// High-intensity blue.
    LightBlue,
    /// High-intensity magenta.
    LightMagenta,
    /// High-intensity cyan.
    LightCyan,
    /// High-intensity white.
    LightWhite,
    /// 216-color (r, g, b ≤ 5) RGB.
    Rgb(u8, u8, u8),
    /// Grayscale (max value: 24)
    Grayscale(u8),
}

use Color::*;

impl Color {
    /// Get the corresponding ANSI value.
    ///
    /// Panics
    /// ======
    ///
    /// This method will panic in debug mode, if `self` is invalid (that is, the values are out of
    /// bound).
    pub fn to_ansi_val(self) -> u8 {
        self.debug_check();

        match self {
            Black => 0x0,
            Red => 0x1,
            Green => 0x2,
            Yellow => 0x3,
            Blue => 0x4,
            Magenta => 0x5,
            Cyan => 0x6,
            White => 0x7,
            LightBlack => 0x8,
            LightRed => 0x9,
            LightGreen => 0xA,
            LightYellow => 0xB,
            LightBlue => 0xC,
            LightMagenta => 0xD,
            LightCyan => 0xE,
            LightWhite => 0xF,
            Rgb(r, g, b) => 16 + 36 * r + 6 * g + b,
            Grayscale(shade) => 0xE8 + shade,
        }
    }

    fn debug_check(self) {
        match self {
            Rgb(r, g, b) => {
                debug_assert!(r <= 5, "Red color fragment (r = {}) is out of bound. Make sure r ≤ 5.", r);
                debug_assert!(g <= 5, "Green color fragment (g = {}) is out of bound. Make sure g ≤ 5.", g);
                debug_assert!(b <= 5, "Blue color fragment (b = {}) is out of bound. Make sure b ≤ 5.", b);
            },
            Grayscale(shade) => {
                // Unfortunately, there are a little less than fifty shades.
                debug_assert!(shade < 24, "Grayscale out of bound (shade = {}). There are only 24 shades of gray.", shade);
            },
            _ => {},
        }
    }
}
