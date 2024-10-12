pub mod pattern;

/// A color value used in pattern definitions
#[derive(Copy, Clone)]
pub enum PatternColor {
    /// A simple solid color
    Solid { color: [u8; 3] },
    /// A solid color with an Alpha channel in [0,255]
    Alpha { color: [u8; 3], alpha: u8 },
    /// A solid color blended with a perlin noise
    Perlin { blendcolor: [u8; 3] },
    /// No color at all - This pixel should be skipped
    None,
    /// A random color in the given range
    Random {
        minvalue: [u8; 3],
        maxvalue: [u8; 3],
    },
}
