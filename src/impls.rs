use crate::color_print::Exeptions;
use crate::RGBA;
use crate::{Color, ColorStandered};
use core::fmt::Display;
use std::sync::RwLock;

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

impl Default for Color {
    fn default() -> Self {
        Self {
            rgb: RwLock::new(None),
            cmyk: RwLock::new(None),
            hsl: RwLock::new(None),
            hsv: RwLock::new(None),
            rgba: RwLock::new(None),
            kind: RwLock::new(ColorStandered::None),
        }
    }
}

// impl ::core::marker::StructuralPartialEq for ColorStandered { }

impl PartialEq for ColorStandered {
    fn eq(&self, other: &ColorStandered) -> bool {
        let __self_tag = core::mem::discriminant(self);
        let __arg1_tag = core::mem::discriminant(other);
        __self_tag == __arg1_tag
    }
}

impl From<RGBA> for (f64, f64, f64) {
    fn from(value: RGBA) -> Self {
        let r = ((value.foreground.0 * value.alpha) + value.background.0) / 2.;
        let g = ((value.foreground.1 * value.alpha) + value.background.1) / 2.;
        let b = ((value.foreground.2 * value.alpha) + value.background.2) / 2.;
        (r, g, b)
    }
}

impl Default for RGBA {
    fn default() -> Self {
        Self {
            background: (0., 0., 0.),
            foreground: (0., 0., 0.),
            alpha: 0.,
        }
    }
}
