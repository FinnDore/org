use serde::{Deserialize, Serialize, Serializer};

use crate::hex;

#[derive(Deserialize, Debug, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Clone for Color {
    fn clone(&self) -> Self {
        *self
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(color: (u8, u8, u8)) -> Self {
        Self {
            r: color.0,
            g: color.1,
            b: color.2,
        }
    }
}

impl Color {
    pub const fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    #[allow(dead_code)]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_hex(hex: &str) -> Result<Self, ()> {
        match hex::hex_to_rgb(hex) {
            Ok((_, color)) => Ok(color),
            Err(_) => Err(()),
        }
    }

    pub const fn white() -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    pub fn increment(&mut self) {
        if self.r < 255 {
            self.r += 1;
        } else {
            if self.g < 255 {
                self.g += 1;
            } else {
                if self.b < 255 {
                    self.b += 1;
                } else {
                    self.b = 0;
                    self.r = 128;
                    self.g = 0;
                }
            }
        }
    }
}

impl Into<String> for &Color {
    fn into(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let color: String = self.into();
        serializer.serialize_str(&color)
    }
}
