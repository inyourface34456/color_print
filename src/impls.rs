#![allow(unused)]

use crate::color_print::NumType;
use crate::color_print::{Color as _, Exeptions};
use crate::{Color as ColorStruct, ColorStandered};
use core::fmt::Display;
use std::cell::Cell;

impl Display for Exeptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncorrectLength(len) => write!(
                f,
                "Expctd a length between 6 and 7 (inclusive), but got length {}",
                len
            ),
            Self::HueOutOfRange(value) => {
                write!(f, "Expectd a value inbetween 0 and 360 but got {}", value)
            }
            Self::SaterationOutOfRange(value) => {
                write!(f, "Expectd a value inbetween 0 and 1 but got {}", value)
            }
            Self::LightnessOutOfRange(value) => {
                write!(f, "Expectd a value inbetween 0 and 1 but got {}", value)
            }
            Self::ValueOutOfRange(value) => {
                write!(f, "Expectd a value inbetween 0 and 1 but got {}", value)
            }
            Self::CyanOutOfRange(value) => {
                write!(f, "Expectd a value inbetween 0 and 1 but got {}", value)
            }
            Self::MagentaOutOfRange(value) => {
                write!(f, "Expectd a value inbetween 0 and 1 but got {}", value)
            }
            Self::YellowOutOfRange(value) => {
                write!(f, "Expectd a value inbetween 0 and 1 but got {}", value)
            }
            Self::BlackOutOfRange(value) => {
                write!(f, "Expectd a value inbetween 0 and 1 but got {}", value)
            }
            Self::RedOutOfRange(value) => {
                write!(f, "Expectd a value inbetween 0 and 255 but got {}", value)
            }
            Self::GreenOutOfRange(value) => {
                write!(f, "Expectd a value inbetween 0 and 255 but got {}", value)
            }
            Self::BlueOutOfRange(value) => {
                write!(f, "Expectd a value inbetween 0 and 255 but got {}", value)
            }
            Self::AlphaOutOfRange(value) => {
                write!(f, "Expectd a value inbetween 0 and 1 but got {}", value)
            }
        }
    }
}

impl std::error::Error for Exeptions {}

impl Default for ColorStruct {
    fn default() -> Self {
        Self {
            rgb: Cell::new(None),
            cmyk: Cell::new(None),
            hsl: Cell::new(None),
            hsv: Cell::new(None),
            kind: Cell::new(ColorStandered::None),
        }
    }
}

impl PartialEq for ColorStandered {
    fn eq(&self, other: &ColorStandered) -> bool {
        let __self_tag = core::mem::discriminant(self);
        let __arg1_tag = core::mem::discriminant(other);
        __self_tag == __arg1_tag
    }
}

impl ColorStruct {
    pub fn new_cmyk(cyan: f64, magenta: f64, yellow: f64, black: f64) -> Result<Self, Exeptions> {
        if !(0. ..=1.).contains(&cyan) {
            return Err(Exeptions::CyanOutOfRange(cyan));
        } else if !(0. ..=1.).contains(&magenta) {
            return Err(Exeptions::MagentaOutOfRange(magenta));
        } else if !(0. ..=1.).contains(&yellow) {
            return Err(Exeptions::YellowOutOfRange(yellow));
        } else if !(0. ..=1.).contains(&black) {
            return Err(Exeptions::BlackOutOfRange(black));
        }

        Ok(Self {
            cmyk: Cell::new(Some((cyan, magenta, yellow, black))),
            kind: Cell::new(ColorStandered::Cmyk),
            ..Self::default()
        })
    }

    pub fn new_hex(value: String) -> Result<Self, Exeptions> {
        if value.len() != 6 && value.len() != 7 {
            return Err(Exeptions::IncorrectLength(
                value.len().try_into().expect("length to large"),
            ));
        }

        let values: [u8; 4] = if value.starts_with('#') {
            let value = &value[1..value.len() - 1];
            i32::from_str_radix(value, 16)
                .expect("invalid syntax")
                .to_le_bytes()
        } else {
            i32::from_str_radix(value.as_str(), 16)
                .expect("invalid syntax")
                .to_le_bytes()
        };

        assert!(values[3] == 0);

        Ok(Self {
            rgb: Cell::new(Some((
                values[0] as NumType,
                values[1] as NumType,
                values[2] as NumType,
            ))),
            kind: Cell::new(ColorStandered::Rgb),
            ..Self::default()
        })
    }

    pub fn new_hsl(hue: f64, sateration: f64, lightness: f64) -> Result<Self, Exeptions> {
        if !(0. ..=360.).contains(&hue) {
            return Err(Exeptions::HueOutOfRange(hue));
        } else if !(0. ..=1.).contains(&sateration) {
            return Err(Exeptions::SaterationOutOfRange(sateration));
        } else if !(0. ..=1.).contains(&lightness) {
            return Err(Exeptions::LightnessOutOfRange(lightness));
        }

        Ok(Self {
            hsl: Cell::new(Some((hue, sateration, lightness))),
            kind: Cell::new(ColorStandered::Hsl),
            ..Self::default()
        })
    }

    pub fn new_hsv(hue: f64, sateration: f64, value: f64) -> Result<Self, Exeptions> {
        if !(0. ..=360.).contains(&hue) {
            return Err(Exeptions::HueOutOfRange(hue));
        } else if !(0. ..=1.).contains(&sateration) {
            return Err(Exeptions::SaterationOutOfRange(sateration));
        } else if !(0. ..=1.).contains(&value) {
            return Err(Exeptions::LightnessOutOfRange(value));
        }

        Ok(Self {
            hsv: Cell::new(Some((hue, sateration, value))),
            kind: Cell::new(ColorStandered::Hsv),
            ..Self::default()
        })
    }

    pub fn new_rgb(red: f64, green: f64, blue: f64) -> Result<Self, Exeptions> {
        if !(0. ..=255.).contains(&red) {
            return Err(Exeptions::RedOutOfRange(red));
        } else if !(0. ..=255.).contains(&green) {
            return Err(Exeptions::GreenOutOfRange(green));
        } else if !(0. ..=255.).contains(&blue) {
            return Err(Exeptions::BlueOutOfRange(blue));
        }

        Ok(Self {
            rgb: Cell::new(Some((red, green, blue))),
            kind: Cell::new(ColorStandered::Rgb),
            ..Self::default()
        })
    }

    pub fn to_string_no_handle(&self, background: Option<Self>) -> String {
        if background.is_none() {
            let forground = self.into_standered(ColorStandered::Rgb);
            return format!(
                "\x1b[38;2;{};{};{}m",
                forground.0.trunc(),
                forground.1.trunc(),
                forground.2.trunc()
            );
        }

        let forground = self.into_standered(ColorStandered::Rgb);
        let background = background.unwrap().into_standered(ColorStandered::Rgb);

        format!(
            "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m",
            forground.0.trunc(),
            forground.1.trunc(),
            forground.2.trunc(),
            background.0.trunc(),
            background.1.trunc(),
            background.2.trunc()
        )
    }

    pub fn as_rgb_with_alpha_no_handle(
        &self,
        to_mix: Self,
        alpha: NumType,
    ) -> Result<Self, Exeptions> {
        if !(0. ..=1.).contains(&alpha) {
            return Err(Exeptions::AlphaOutOfRange(alpha));
        }

        let fg = self.into_standered(ColorStandered::Rgb);
        let bg = to_mix.into_standered(ColorStandered::Rgb);

        let r = ((fg.0 * alpha) + bg.0) / 2.;
        let g = ((fg.1 * alpha) + bg.1) / 2.;
        let b = ((fg.2 * alpha) + bg.2) / 2.;

        Self::new_rgb(r, g, b)
    }
}
