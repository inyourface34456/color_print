use crate::color_print::Exeptions;
use core::fmt::Display;

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
        }
    }
}
