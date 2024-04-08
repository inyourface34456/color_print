mod impls;
mod rgba;
mod utils;

use crate::color_print::{ColorStandered, Exeptions, NumType};
use rgba::RGBA;
use std::sync::RwLock;
use utils::{from_rgb, to_rgb};

wai_bindgen_rust::export!("color_print.wai");

type Wrapper<T> = RwLock<Option<T>>;

struct ColorPrint;
impl crate::color_print::ColorPrint for ColorPrint {}

struct Color {
    rgb: Wrapper<(NumType, NumType, NumType)>,
    cmyk: Wrapper<(NumType, NumType, NumType, NumType)>,
    hsl: Wrapper<(NumType, NumType, NumType)>,
    hsv: Wrapper<(NumType, NumType, NumType)>,
    rgba: Wrapper<RGBA>,
    kind: RwLock<ColorStandered>,
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
            cmyk: RwLock::new(Some((cyan, magenta, yellow, black))),
            kind: RwLock::new(ColorStandered::Cmyk),
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
            kind: RwLock::new(ColorStandered::Rgb),
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
            kind: RwLock::new(ColorStandered::Hsl),
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
            kind: RwLock::new(ColorStandered::Hsv),
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
            kind: RwLock::new(ColorStandered::Rgb),
            ..Self::default()
        }
        .into())
    }

    fn from_rgba(
        foreground: (f64, f64, f64),
        background: (f64, f64, f64),
        alpha: f64,
    ) -> Result<wai_bindgen_rust::Handle<crate::Color>, color_print::Exeptions> {
        if !(0. ..=255.).contains(&foreground.0) {
            return Err(Exeptions::RedOutOfRange(foreground.0));
        } else if !(0. ..=255.).contains(&foreground.1) {
            return Err(Exeptions::GreenOutOfRange(foreground.1));
        } else if !(0. ..=255.).contains(&foreground.2) {
            return Err(Exeptions::BlueOutOfRange(foreground.2));
        } else if !(0. ..=255.).contains(&background.0) {
            return Err(Exeptions::RedOutOfRange(background.0));
        } else if !(0. ..=255.).contains(&background.1) {
            return Err(Exeptions::GreenOutOfRange(background.1));
        } else if !(0. ..=255.).contains(&background.2) {
            return Err(Exeptions::BlueOutOfRange(background.2));
        } else if !(0. ..=1.).contains(&alpha) {
            return Err(Exeptions::AlphaOutOfRange(alpha));
        }

        Ok(Self {
            rgba: RwLock::new(Some(RGBA::new(foreground, background, alpha))),
            kind: RwLock::new(ColorStandered::Rgba),
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
        *(self.cmyk.read().ok()?)
    }

    fn to_hsl(&self) -> Option<(f64, f64, f64)> {
        *(self.hsl.read().ok()?)
    }

    fn to_hsv(&self) -> Option<(f64, f64, f64)> {
        *(self.hsv.read().ok()?)
    }

    fn to_rgb(&self) -> Option<(f64, f64, f64)> {
        *(self.rgb.read().ok()?)
    }

    fn to_rgba(&self) -> Option<(f64, f64, f64)> {
        let rgba = self.rgba.read().ok()?;

        (*rgba).map(|dat| dat.into())
    }

    fn as_standered(&self, standered: ColorStandered) {
        let kind = self.get_standered();

        if standered == kind
            || standered == ColorStandered::None
            || standered == ColorStandered::Rgba
        {
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
                ColorStandered::Rgba => {
                    let origin: (f64, f64, f64) =
                        self.rgba.write().unwrap().take().unwrap_or_default().into();
                    let cmyk = from_rgb::rgb_to_cmyk(origin.0, origin.1, origin.2);

                    let mut rgba_ref = self.rgba.write().unwrap();
                    rgba_ref.take();

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
                ColorStandered::Rgba => {
                    let origin: (f64, f64, f64) =
                        self.rgba.write().unwrap().take().unwrap_or_default().into();
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
                ColorStandered::Rgba => {
                    let origin: (f64, f64, f64) =
                        self.rgba.write().unwrap().take().unwrap_or_default().into();
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
                ColorStandered::Rgba => {
                    let origin: (f64, f64, f64) =
                        self.rgba.write().unwrap().take().unwrap_or_default().into();
                    let rgb = origin;

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

    fn to_string(&self) -> String {
        String::new()
    }
}
