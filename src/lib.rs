use crate::color_print::{Exeptions, NumType};
use std::sync::RwLock;

mod impls;
mod utils;

wai_bindgen_rust::export!("color_print.wai");

type Wrapper<T> = RwLock<Option<T>>;

struct ColorPrint;
impl crate::color_print::ColorPrint for ColorPrint {}

struct Color {
    rgb: Wrapper<(NumType, NumType, NumType)>,
    cymk: Wrapper<(NumType, NumType, NumType, NumType)>,
    hsl: Wrapper<(NumType, NumType, NumType)>,
    hsv: Wrapper<(NumType, NumType, NumType)>,
}

impl crate::color_print::Color for Color {
    fn from_cmyk(
        cyan: f64,
        magenta: f64,
        yellow: f64,
        black: f64,
    ) -> Result<wai_bindgen_rust::Handle<crate::Color>, color_print::Exeptions> {
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
            cymk: RwLock::new(Some((cyan, magenta, yellow, black))),
            ..Self::default()
        }
        .into())
    }

    fn from_hex(
        value: String,
    ) -> Result<wai_bindgen_rust::Handle<crate::Color>, color_print::Exeptions> {
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
            rgb: RwLock::new(Some((
                values[0] as NumType,
                values[1] as NumType,
                values[2] as NumType,
            ))),
            ..Self::default()
        }
        .into())
    }

    fn from_hsl(
        hue: f64,
        sateration: f64,
        lightness: f64,
    ) -> Result<wai_bindgen_rust::Handle<crate::Color>, color_print::Exeptions> {
        if !(0. ..=360.).contains(&hue) {
            return Err(Exeptions::HueOutOfRange(hue));
        } else if !(0. ..=1.).contains(&sateration) {
            return Err(Exeptions::SaterationOutOfRange(sateration));
        } else if !(0. ..=1.).contains(&lightness) {
            return Err(Exeptions::LightnessOutOfRange(lightness));
        }

        Ok(Self {
            hsl: RwLock::new(Some((hue, sateration, lightness))),
            ..Self::default()
        }
        .into())
    }

    fn from_hsv(
        hue: f64,
        sateration: f64,
        value: f64,
    ) -> Result<wai_bindgen_rust::Handle<crate::Color>, color_print::Exeptions> {
        if !(0. ..=360.).contains(&hue) {
            return Err(Exeptions::HueOutOfRange(hue));
        } else if !(0. ..=1.).contains(&sateration) {
            return Err(Exeptions::SaterationOutOfRange(sateration));
        } else if !(0. ..=1.).contains(&value) {
            return Err(Exeptions::LightnessOutOfRange(value));
        }

        Ok(Self {
            hsv: RwLock::new(Some((hue, sateration, value))),
            ..Self::default()
        }
        .into())
    }

    fn from_rgb(
        red: f64,
        green: f64,
        blue: f64,
    ) -> Result<wai_bindgen_rust::Handle<crate::Color>, color_print::Exeptions> {
        if !(0. ..=255.).contains(&red) {
            return Err(Exeptions::RedOutOfRange(red));
        } else if !(0. ..=255.).contains(&green) {
            return Err(Exeptions::GreenOutOfRange(green));
        } else if !(0. ..=255.).contains(&blue) {
            return Err(Exeptions::BlueOutOfRange(blue));
        }

        Ok(Self {
            rgb: RwLock::new(Some((red, green, blue))),
            ..Self::default()
        }
        .into())
    }

    fn new(
        red: f64,
        green: f64,
        blue: f64,
    ) -> Result<wai_bindgen_rust::Handle<crate::Color>, color_print::Exeptions> {
        Self::from_rgb(red, green, blue)
    }

    fn to_cmyk(&self) -> Option<(f64, f64, f64, f64)> {
        let cymk = self.cymk.read().ok()?;

        *cymk
    }

    // fn to_hex(&self,) -> String {

    // }

    fn to_hsl(&self) -> Option<(f64, f64, f64)> {
        let cymk = self.hsl.read().ok()?;

        *cymk
    }
    fn to_hsv(&self) -> Option<(f64, f64, f64)> {
        let cymk = self.hsv.read().ok()?;

        *cymk
    }

    fn to_rgb(&self) -> Option<(f64, f64, f64)> {
        let cymk = self.rgb.read().ok()?;

        *cymk
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            rgb: RwLock::new(None),
            cymk: RwLock::new(None),
            hsl: RwLock::new(None),
            hsv: RwLock::new(None),
        }
    }
}
