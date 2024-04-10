use crate::color_print::{ColorStandered, Exeptions, NumType};
use crate::utils::types::*;
use crate::utils::{from_rgb, to_rgb};
use std::sync::RwLock;
use wai_bindgen_rust::Handle;

pub struct Color {
    pub rgb: Wrapper<(NumType, NumType, NumType)>,
    pub cmyk: Wrapper<(NumType, NumType, NumType, NumType)>,
    pub hsl: Wrapper<(NumType, NumType, NumType)>,
    pub hsv: Wrapper<(NumType, NumType, NumType)>,
    pub kind: RwLock<ColorStandered>,
}

impl crate::color_print::Color for Color {
    fn from_cmyk(cyan: f64, magenta: f64, yellow: f64, black: f64) -> NewColorResult {
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
            cmyk: RwLock::new(Some((cyan, magenta, yellow, black))),
            kind: RwLock::new(ColorStandered::Cmyk),
            ..Self::default()
        }
        .into())
    }

    fn from_hex(value: String) -> NewColorResult {
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
            kind: RwLock::new(ColorStandered::Rgb),
            ..Self::default()
        }
        .into())
    }

    fn from_hsl(hue: f64, sateration: f64, lightness: f64) -> NewColorResult {
        if !(0. ..=360.).contains(&hue) {
            return Err(Exeptions::HueOutOfRange(hue));
        } else if !(0. ..=1.).contains(&sateration) {
            return Err(Exeptions::SaterationOutOfRange(sateration));
        } else if !(0. ..=1.).contains(&lightness) {
            return Err(Exeptions::LightnessOutOfRange(lightness));
        }

        Ok(Self {
            hsl: RwLock::new(Some((hue, sateration, lightness))),
            kind: RwLock::new(ColorStandered::Hsl),
            ..Self::default()
        }
        .into())
    }

    fn from_hsv(hue: f64, sateration: f64, value: f64) -> NewColorResult {
        if !(0. ..=360.).contains(&hue) {
            return Err(Exeptions::HueOutOfRange(hue));
        } else if !(0. ..=1.).contains(&sateration) {
            return Err(Exeptions::SaterationOutOfRange(sateration));
        } else if !(0. ..=1.).contains(&value) {
            return Err(Exeptions::LightnessOutOfRange(value));
        }

        Ok(Self {
            hsv: RwLock::new(Some((hue, sateration, value))),
            kind: RwLock::new(ColorStandered::Hsv),
            ..Self::default()
        }
        .into())
    }

    fn from_rgb(red: f64, green: f64, blue: f64) -> NewColorResult {
        if !(0. ..=255.).contains(&red) {
            return Err(Exeptions::RedOutOfRange(red));
        } else if !(0. ..=255.).contains(&green) {
            return Err(Exeptions::GreenOutOfRange(green));
        } else if !(0. ..=255.).contains(&blue) {
            return Err(Exeptions::BlueOutOfRange(blue));
        }

        Ok(Self {
            rgb: RwLock::new(Some((red, green, blue))),
            kind: RwLock::new(ColorStandered::Rgb),
            ..Self::default()
        }
        .into())
    }

    fn new(red: f64, green: f64, blue: f64) -> NewColorResult {
        Self::from_rgb(red, green, blue)
    }

    fn to_cmyk(&self) -> Option<CMYK> {
        *(self.cmyk.read().ok()?)
    }

    fn to_hsl(&self) -> Option<HSL> {
        *(self.hsl.read().ok()?)
    }

    fn to_hsv(&self) -> Option<HSV> {
        *(self.hsv.read().ok()?)
    }

    fn to_rgb(&self) -> Option<RGB> {
        *(self.rgb.read().ok()?)
    }

    fn as_standered(&self, standered: ColorStandered) {
        let kind = self.get_standered();

        if standered == kind || standered == ColorStandered::None {
            return;
        }

        match standered {
            ColorStandered::Cmyk => match kind {
                ColorStandered::Hsl => {
                    let hsl = self.hsl.write().unwrap().take().unwrap_or_default();
                    let rgb = to_rgb::hsl_to_rgb(hsl.0, hsl.1, hsl.2);
                    let cmyk = from_rgb::rgb_to_cmyk(rgb.0, rgb.1, rgb.2);

                    let mut cmyk_ref = self.cmyk.write().unwrap();
                    cmyk_ref.replace(cmyk);
                }
                ColorStandered::Hsv => {
                    let hsv = self.hsv.write().unwrap().take().unwrap_or_default();
                    let rgb = to_rgb::hsl_to_rgb(hsv.0, hsv.1, hsv.2);
                    let cmyk = from_rgb::rgb_to_cmyk(rgb.0, rgb.1, rgb.2);

                    let mut cmyk_ref = self.cmyk.write().unwrap();
                    cmyk_ref.replace(cmyk);
                }
                ColorStandered::Rgb => {
                    let rgb = self.rgb.write().unwrap().take().unwrap_or_default();
                    let cmyk = from_rgb::rgb_to_cmyk(rgb.0, rgb.1, rgb.2);

                    let mut cmyk_ref = self.cmyk.write().unwrap();
                    cmyk_ref.replace(cmyk);
                }
                _ => {}
            },
            ColorStandered::Hsl => match kind {
                ColorStandered::Cmyk => {
                    let origin = self.cmyk.write().unwrap().take().unwrap_or_default();
                    let rgb = to_rgb::cmyk_to_rgb(origin.0, origin.1, origin.2, origin.3);
                    let end = from_rgb::rgb_to_hsl(rgb.0, rgb.1, rgb.2);

                    let mut end_ref = self.hsl.write().unwrap();
                    end_ref.replace(end);
                }
                ColorStandered::Hsv => {
                    let origin = self.hsv.write().unwrap().take().unwrap_or_default();
                    let rgb = to_rgb::hsv_to_rgb(origin.0, origin.1, origin.2);
                    let end = from_rgb::rgb_to_hsv(rgb.0, rgb.1, rgb.2);

                    let mut end_ref = self.hsl.write().unwrap();
                    end_ref.replace(end);
                }
                ColorStandered::Rgb => {
                    let origin = self.rgb.write().unwrap().take().unwrap_or_default();
                    let rgb = origin;
                    let end = from_rgb::rgb_to_hsl(rgb.0, rgb.1, rgb.2);

                    let mut end_ref = self.hsl.write().unwrap();
                    end_ref.replace(end);
                }
                _ => {}
            },
            ColorStandered::Hsv => match kind {
                ColorStandered::Cmyk => {
                    let origin = self.cmyk.write().unwrap().take().unwrap_or_default();
                    let rgb = to_rgb::cmyk_to_rgb(origin.0, origin.1, origin.2, origin.3);
                    let end = from_rgb::rgb_to_hsv(rgb.0, rgb.1, rgb.2);

                    let mut end_ref = self.hsv.write().unwrap();
                    end_ref.replace(end);
                }
                ColorStandered::Hsl => {
                    let origin = self.hsl.write().unwrap().take().unwrap_or_default();
                    let rgb = to_rgb::hsl_to_rgb(origin.0, origin.1, origin.2);
                    let end = from_rgb::rgb_to_hsv(rgb.0, rgb.1, rgb.2);

                    let mut end_ref = self.hsv.write().unwrap();
                    end_ref.replace(end);
                }
                ColorStandered::Rgb => {
                    let origin = self.rgb.write().unwrap().take().unwrap_or_default();
                    let rgb = origin;
                    let end = from_rgb::rgb_to_hsv(rgb.0, rgb.1, rgb.2);

                    let mut end_ref = self.hsv.write().unwrap();
                    end_ref.replace(end);
                }
                _ => {}
            },
            ColorStandered::Rgb => match kind {
                ColorStandered::Cmyk => {
                    let origin = self.cmyk.write().unwrap().take().unwrap_or_default();
                    let rgb = to_rgb::cmyk_to_rgb(origin.0, origin.1, origin.2, origin.3);

                    let mut end_ref = self.rgb.write().unwrap();
                    end_ref.replace(rgb);
                }
                ColorStandered::Hsl => {
                    let origin = self.hsl.write().unwrap().take().unwrap_or_default();
                    let rgb = to_rgb::hsl_to_rgb(origin.0, origin.1, origin.2);

                    let mut end_ref = self.rgb.write().unwrap();
                    end_ref.replace(rgb);
                }
                ColorStandered::Hsv => {
                    let origin = self.hsv.write().unwrap().take().unwrap_or_default();
                    let rgb = to_rgb::hsv_to_rgb(origin.0, origin.1, origin.2);

                    let mut end_ref = self.rgb.write().unwrap();
                    end_ref.replace(rgb);
                }
                _ => {}
            },
            _ => {}
        }

        *self.kind.write().unwrap() = standered
    }

    fn get_standered(&self) -> ColorStandered {
        *self.kind.read().unwrap()
    }

    fn get_internel_color(&self) -> (NumType, NumType, NumType, Option<NumType>) {
        match self.get_standered() {
            ColorStandered::Rgb => {
                let rgb = self.to_rgb().unwrap();
                (rgb.0, rgb.1, rgb.2, None)
            }
            ColorStandered::Hsl => {
                let hsl = self.to_hsl().unwrap();
                (hsl.0, hsl.1, hsl.2, None)
            }
            ColorStandered::Hsv => {
                let hsv = self.to_hsv().unwrap();
                (hsv.0, hsv.1, hsv.2, None)
            }
            ColorStandered::Cmyk => {
                let cmyk = self.to_cmyk().unwrap();
                (cmyk.0, cmyk.1, cmyk.2, Some(cmyk.3))
            }
            ColorStandered::None => (0., 0., 0., None),
        }
    }

    fn into_rgb(&self) -> RGB {
        let color = self.get_internel_color();
        let rgb: (f64, f64, f64);

        if let Some(black) = color.3 {
            let cmyk = (color.0, color.1, color.2, black);

            rgb = to_rgb::cmyk_to_rgb(cmyk.0, cmyk.1, cmyk.2, cmyk.3);
        } else {
            match self.get_standered() {
                ColorStandered::Rgb => rgb = (color.0, color.1, color.2),
                ColorStandered::Hsl => rgb = to_rgb::hsl_to_rgb(color.0, color.1, color.2),
                ColorStandered::Hsv => rgb = to_rgb::hsv_to_rgb(color.0, color.1, color.2),
                _ => rgb = (0., 0., 0.),
            };
        }
        rgb
    }

    fn to_string(&self, background: Option<Handle<Self>>) -> String {
        if background.is_none() {
            return format!("{}", self);
        }

        let forground = self.into_rgb();
        let background = background.unwrap().into_rgb();

        format!(
            "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m",
            forground.0, forground.1, forground.2, background.0, background.1, background.2
        )
    }

    fn into_rgb_with_alpha(&self, to_mix: Handle<Color>, alpha: NumType) -> NewColorResult {
        if (0. ..=1.).contains(&alpha) {
            return Err(Exeptions::AlphaOutOfRange(alpha))
        }
        
        let fg = self.into_rgb();
        let bg = to_mix.into_rgb();

        let r = ((fg.0 * alpha) + bg.0) / 2.;
        let g = ((fg.1 * alpha) + bg.1) / 2.;
        let b = ((fg.2 * alpha) + bg.2) / 2.;

        Color::new(r, g, b)
    }
}
