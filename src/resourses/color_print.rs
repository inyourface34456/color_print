use crate::color_print::{Color as _, NumType};
use crate::resourses::Color;
use crate::utils::types::NewColorResult;

pub struct ColorPrint;
impl crate::color_print::ColorPrint for ColorPrint {
    fn new(red: NumType, green: NumType, blue: NumType) -> NewColorResult {
        Color::new(red, green, blue)
    }

    fn from_cmyk(
        cyan: NumType,
        magenta: NumType,
        yellow: NumType,
        black: NumType,
    ) -> NewColorResult {
        Color::from_cmyk(cyan, magenta, yellow, black)
    }

    fn from_hex(value: String) -> NewColorResult {
        Color::from_hex(value)
    }

    fn from_hsl(hue: NumType, sateration: NumType, lightness: NumType) -> NewColorResult {
        Color::from_hsl(hue, sateration, lightness)
    }

    fn from_hsv(hue: NumType, sateration: NumType, value: NumType) -> NewColorResult {
        Color::from_hsv(hue, sateration, value)
    }

    fn from_rgb(red: NumType, green: NumType, blue: NumType) -> NewColorResult {
        Color::from_rgb(red, green, blue)
    }
}
