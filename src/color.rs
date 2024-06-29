use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use crate::vec::Vec3;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_normalized_f32(
        r: f32,
        g: f32,
        b: f32,
    ) -> Result<Self, &'static str> {
        if (r < 0f32 || r > 1f32)
            || (g < 0f32 || g > 1f32)
            || (b < 0f32 || b > 1f32)
        {
            return Err("values must be between `0` and `1`");
        }

        let r = (255.999f32 * r) as u8;
        let g = (255.999f32 * g) as u8;
        let b = (255.999f32 * b) as u8;

        Ok(Self::new(r, g, b))
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(hex) = s.strip_prefix("#") else {
            return Err("invalid str, color hex must start with `#`");
        };

        let r = match hex.get(0..=1).map(|h| u8::from_str_radix(h, 16)) {
            Some(Ok(r)) => r,
            Some(Err(_)) => {
                return Err("invalid str, could not parse into `u8`")
            }
            None => return Err("invalid str, red part is missing"),
        };
        let g = match hex.get(2..=3).map(|h| u8::from_str_radix(h, 16)) {
            Some(Ok(r)) => r,
            Some(Err(_)) => {
                return Err("invalid str, could not parse into `u8`")
            }
            None => return Err("invalid str, red part is missing"),
        };
        let b = match hex.get(4..=5).map(|h| u8::from_str_radix(h, 16)) {
            Some(Ok(r)) => r,
            Some(Err(_)) => {
                return Err("invalid str, could not parse into `u8`")
            }
            None => return Err("invalid str, red part is missing"),
        };

        Ok(Self { r, g, b })
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

impl TryFrom<Vec3> for Color {
    type Error = &'static str;

    fn try_from(value: Vec3) -> Result<Self, Self::Error> {
        Self::from_normalized_f32(value.i, value.j, value.k)
    }
}
