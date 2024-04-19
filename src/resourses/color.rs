use crate::color_print::{ColorStandered, Exeptions, NumType};
use crate::utils::types::*;
use crate::utils::{from_rgb, to_rgb};
use std::cell::Cell;
use wai_bindgen_rust::Handle;

pub struct Color {
    pub rgb: Wrapper<(NumType, NumType, NumType)>,
    pub cmyk: Wrapper<(NumType, NumType, NumType, NumType)>,
    pub hsl: Wrapper<(NumType, NumType, NumType)>,
    pub hsv: Wrapper<(NumType, NumType, NumType)>,
    pub kind: Cell<ColorStandered>,
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
            cmyk: Cell::new(Some((cyan, magenta, yellow, black))),
            kind: Cell::new(ColorStandered::Cmyk),
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
            rgb: Cell::new(Some((
                values[0] as NumType,
                values[1] as NumType,
                values[2] as NumType,
            ))),
            kind: Cell::new(ColorStandered::Rgb),
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
            hsl: Cell::new(Some((hue, sateration, lightness))),
            kind: Cell::new(ColorStandered::Hsl),
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
            hsv: Cell::new(Some((hue, sateration, value))),
            kind: Cell::new(ColorStandered::Hsv),
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
            rgb: Cell::new(Some((red, green, blue))),
            kind: Cell::new(ColorStandered::Rgb),
            ..Self::default()
        }
        .into())
    }

    fn new(red: f64, green: f64, blue: f64) -> NewColorResult {
        Self::from_rgb(red, green, blue)
    }

    fn new_unchecked(red: f64, green: f64, blue: f64) -> Handle<Color> {
        Self {
            rgb: Cell::new(Some((red, green, blue))),
            kind: Cell::new(ColorStandered::Rgb),
            ..Self::default()
        }
        .into()
    }

    fn to_cmyk(&self) -> Option<CMYK> {
        self.cmyk.get()
    }

    fn to_hsl(&self) -> Option<HSL> {
        self.hsl.get()
    }

    fn to_hsv(&self) -> Option<HSV> {
        self.hsv.get()
    }

    fn to_rgb(&self) -> Option<RGB> {
        self.rgb.get()
    }

    fn as_standered(&self, standered: ColorStandered) {
        let kind = self.get_standered();

        if standered == kind || standered == ColorStandered::None {
            return;
        }

        match standered {
            ColorStandered::Cmyk => match kind {
                ColorStandered::Hsl => {
                    let hsl = self.hsl.get().unwrap_or_default();
                    let rgb = to_rgb::hsl_to_rgb(hsl.0, hsl.1, hsl.2);
                    let cmyk = from_rgb::rgb_to_cmyk(rgb.0, rgb.1, rgb.2);

                    self.cmyk.replace(Some(cmyk));
                    //cmyk_ref.replace(cmyk);
                }
                ColorStandered::Hsv => {
                    let hsv = self.hsv.get().unwrap_or_default();
                    let rgb = to_rgb::hsv_to_rgb(hsv.0, hsv.1, hsv.2);
                    let cmyk = from_rgb::rgb_to_cmyk(rgb.0, rgb.1, rgb.2);

                    self.cmyk.replace(Some(cmyk));
                }
                ColorStandered::Rgb => {
                    let rgb = self.rgb.get().unwrap_or_default();
                    let cmyk = from_rgb::rgb_to_cmyk(rgb.0, rgb.1, rgb.2);

                    self.cmyk.replace(Some(cmyk));
                }
                _ => {}
            },
            ColorStandered::Hsl => match kind {
                ColorStandered::Cmyk => {
                    let origin = self.cmyk.get().unwrap_or_default();
                    let rgb = to_rgb::cmyk_to_rgb(origin.0, origin.1, origin.2, origin.3);
                    let end = from_rgb::rgb_to_hsl(rgb.0, rgb.1, rgb.2);

                    self.hsl.replace(Some(end));
                }
                ColorStandered::Hsv => {
                    let origin = self.hsv.get().unwrap_or_default();
                    let rgb = to_rgb::hsv_to_rgb(origin.0, origin.1, origin.2);
                    let end = from_rgb::rgb_to_hsl(rgb.0, rgb.1, rgb.2);

                    self.hsl.replace(Some(end));
                }
                ColorStandered::Rgb => {
                    let origin = self.rgb.get().unwrap_or_default();
                    let rgb = origin;
                    let end = from_rgb::rgb_to_hsl(rgb.0, rgb.1, rgb.2);

                    self.hsl.replace(Some(end));
                }
                _ => {}
            },
            ColorStandered::Hsv => match kind {
                ColorStandered::Cmyk => {
                    let origin = self.cmyk.get().unwrap_or_default();
                    let rgb = to_rgb::cmyk_to_rgb(origin.0, origin.1, origin.2, origin.3);
                    let end = from_rgb::rgb_to_hsv(rgb.0, rgb.1, rgb.2);

                    self.hsv.replace(Some(end));
                }
                ColorStandered::Hsl => {
                    let origin = self.hsl.get().unwrap_or_default();
                    let rgb = to_rgb::hsl_to_rgb(origin.0, origin.1, origin.2);
                    let end = from_rgb::rgb_to_hsv(rgb.0, rgb.1, rgb.2);

                    self.hsv.replace(Some(end));
                }
                ColorStandered::Rgb => {
                    let origin = self.rgb.get().unwrap_or_default();
                    let rgb = origin;
                    let end = from_rgb::rgb_to_hsv(rgb.0, rgb.1, rgb.2);

                    self.hsv.replace(Some(end));
                }
                _ => {}
            },
            ColorStandered::Rgb => match kind {
                ColorStandered::Cmyk => {
                    let origin = self.cmyk.get().unwrap_or_default();
                    let rgb = to_rgb::cmyk_to_rgb(origin.0, origin.1, origin.2, origin.3);

                    self.rgb.replace(Some(rgb));
                }
                ColorStandered::Hsl => {
                    let origin = self.hsl.get().unwrap_or_default();
                    let rgb = to_rgb::hsl_to_rgb(origin.0, origin.1, origin.2);

                    self.rgb.replace(Some(rgb));
                }
                ColorStandered::Hsv => {
                    let origin = self.hsv.get().unwrap_or_default();
                    let rgb = to_rgb::hsv_to_rgb(origin.0, origin.1, origin.2);

                    self.rgb.replace(Some(rgb));
                }
                _ => {}
            },
            _ => {}
        }

        self.kind.replace(standered);
    }

    fn get_standered(&self) -> ColorStandered {
        self.kind.get()
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

    fn into_standered(
        &self,
        standered: ColorStandered,
    ) -> (NumType, NumType, NumType, Option<NumType>) {
        let kind = self.get_standered();

        if standered == kind || standered == ColorStandered::None {
            return match standered {
                ColorStandered::Cmyk => {
                    let color = self.to_cmyk().unwrap();
                    (color.0, color.1, color.2, Some(color.3))
                }
                ColorStandered::Rgb => {
                    let value = self.to_rgb().unwrap();
                    (value.0, value.1, value.2, None)
                }
                ColorStandered::Hsl => {
                    let value = self.to_hsl().unwrap();
                    (value.0, value.1, value.2, None)
                }
                ColorStandered::Hsv => {
                    let value = self.to_hsv().unwrap();
                    (value.0, value.1, value.2, None)
                }
                ColorStandered::None => (0., 0., 0., None),
            };
        }

        match standered {
            ColorStandered::Cmyk => match kind {
                ColorStandered::Hsl => {
                    let hsl = self.hsl.get().unwrap_or_default();
                    let rgb = to_rgb::hsl_to_rgb(hsl.0, hsl.1, hsl.2);
                    let cmyk = from_rgb::rgb_to_cmyk(rgb.0, rgb.1, rgb.2);

                    (cmyk.0, cmyk.1, cmyk.2, Some(cmyk.3))
                }
                ColorStandered::Hsv => {
                    let hsv = self.hsv.get().unwrap_or_default();
                    let rgb = to_rgb::hsv_to_rgb(hsv.0, hsv.1, hsv.2);
                    let cmyk = from_rgb::rgb_to_cmyk(rgb.0, rgb.1, rgb.2);

                    (cmyk.0, cmyk.1, cmyk.2, Some(cmyk.3))
                }
                ColorStandered::Rgb => {
                    let rgb = self.rgb.get().unwrap_or_default();
                    let cmyk = from_rgb::rgb_to_cmyk(rgb.0, rgb.1, rgb.2);

                    (cmyk.0, cmyk.1, cmyk.2, Some(cmyk.3))
                }
                _ => (0., 0., 0., None),
            },
            ColorStandered::Hsl => match kind {
                ColorStandered::Cmyk => {
                    let origin = self.cmyk.get().unwrap_or_default();
                    let rgb = to_rgb::cmyk_to_rgb(origin.0, origin.1, origin.2, origin.3);
                    let end = from_rgb::rgb_to_hsl(rgb.0, rgb.1, rgb.2);

                    (end.0, end.1, end.2, None)
                }
                ColorStandered::Hsv => {
                    let origin = self.hsv.get().unwrap_or_default();
                    let rgb = to_rgb::hsv_to_rgb(origin.0, origin.1, origin.2);
                    let end = from_rgb::rgb_to_hsl(rgb.0, rgb.1, rgb.2);

                    (end.0, end.1, end.2, None)
                }
                ColorStandered::Rgb => {
                    let origin = self.rgb.get().unwrap_or_default();
                    let rgb = origin;
                    let end = from_rgb::rgb_to_hsl(rgb.0, rgb.1, rgb.2);

                    (end.0, end.1, end.2, None)
                }
                _ => (0., 0., 0., None),
            },
            ColorStandered::Hsv => match kind {
                ColorStandered::Cmyk => {
                    let origin = self.cmyk.get().unwrap_or_default();
                    let rgb = to_rgb::cmyk_to_rgb(origin.0, origin.1, origin.2, origin.3);
                    let end = from_rgb::rgb_to_hsv(rgb.0, rgb.1, rgb.2);

                    (end.0, end.1, end.2, None)
                }
                ColorStandered::Hsl => {
                    let origin = self.hsl.get().unwrap_or_default();
                    let rgb = to_rgb::hsl_to_rgb(origin.0, origin.1, origin.2);
                    let end = from_rgb::rgb_to_hsv(rgb.0, rgb.1, rgb.2);

                    (end.0, end.1, end.2, None)
                }
                ColorStandered::Rgb => {
                    let origin = self.rgb.get().unwrap_or_default();
                    let rgb = origin;
                    let end = from_rgb::rgb_to_hsv(rgb.0, rgb.1, rgb.2);

                    (end.0, end.1, end.2, None)
                }
                _ => (0., 0., 0., None),
            },
            ColorStandered::Rgb => match kind {
                ColorStandered::Cmyk => {
                    let origin = self.cmyk.get().unwrap_or_default();
                    let rgb = to_rgb::cmyk_to_rgb(origin.0, origin.1, origin.2, origin.3);

                    (rgb.0, rgb.1, rgb.2, None)
                }
                ColorStandered::Hsl => {
                    let origin = self.hsl.get().unwrap_or_default();
                    let rgb = to_rgb::hsl_to_rgb(origin.0, origin.1, origin.2);

                    (rgb.0, rgb.1, rgb.2, None)
                }
                ColorStandered::Hsv => {
                    let origin = self.hsv.get().unwrap_or_default();
                    let rgb = to_rgb::hsv_to_rgb(origin.0, origin.1, origin.2);

                    (rgb.0, rgb.1, rgb.2, None)
                }
                _ => (0., 0., 0., None),
            },
            _ => (0., 0., 0., None),
        }
    }

    fn to_string(&self, background: Option<Handle<Self>>) -> String {
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

    fn into_rgb_with_alpha(&self, to_mix: Handle<Color>, alpha: NumType) -> NewColorResult {
        if !(0. ..=1.).contains(&alpha) {
            return Err(Exeptions::AlphaOutOfRange(alpha));
        }

        let fg = self.into_standered(ColorStandered::Rgb);
        let bg = to_mix.into_standered(ColorStandered::Rgb);

        let r = ((fg.0 * alpha) + bg.0) / 2.;
        let g = ((fg.1 * alpha) + bg.1) / 2.;
        let b = ((fg.2 * alpha) + bg.2) / 2.;

        Color::new(r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use crate::color_print::Color as _;
    use crate::color_print::ColorStandered;
    use crate::resourses::Color;
    use crate::utils::{from_rgb, to_rgb};

    fn into(dat: (f64, f64, f64, f64)) -> (f64, f64, f64, Option<f64>) {
        (dat.0, dat.1, dat.2, Some(dat.3))
    }

    #[test]
    fn to_string_forground_only() {
        let foreground = Color::new_rgb(10.2, 20.3, 30.4).unwrap();

        assert_eq!(
            foreground.to_string(None),
            format!("\x1b[38;2;{};{};{}m", 10, 20, 30)
        )
    }

    #[test]
    fn to_string_forground_background() {
        let foreground = Color::new_rgb(10.2, 20.3, 30.4).unwrap();
        let background = Color::new_rgb(40.2, 50.3, 60.4).unwrap();

        assert_eq!(
            foreground.to_string_no_handle(Some(background)),
            format!(
                "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m",
                10, 20, 30, 40, 50, 60
            )
        )
    }

    #[test]
    fn get_standered() {
        let cymk = Color::new_cmyk(0.1, 0.1, 0.1, 0.1).unwrap();
        let rgb = Color::new_rgb(10.2, 20.3, 30.4).unwrap();
        let hsl = Color::new_hsl(0.1, 0.2, 0.3).unwrap();
        let hsv = Color::new_hsv(0.1, 0.2, 0.3).unwrap();

        assert_eq!(cymk.get_standered(), ColorStandered::Cmyk);
        assert_eq!(rgb.get_standered(), ColorStandered::Rgb);
        assert_eq!(hsl.get_standered(), ColorStandered::Hsl);
        assert_eq!(hsv.get_standered(), ColorStandered::Hsv);
    }

    #[test]
    fn to_standered() {
        let cymk = Color::new_cmyk(0.1, 0.2, 0.3, 0.4).unwrap();
        let rgb = Color::new_rgb(10.2, 20.3, 30.4).unwrap();
        let hsl = Color::new_hsl(0.1, 0.2, 0.3).unwrap();
        let hsv = Color::new_hsv(0.1, 0.2, 0.3).unwrap();

        assert_eq!(cymk.to_cmyk().unwrap(), (0.1, 0.2, 0.3, 0.4));
        assert_eq!(rgb.to_rgb().unwrap(), (10.2, 20.3, 30.4));
        assert_eq!(hsl.to_hsl().unwrap(), (0.1, 0.2, 0.3));
        assert_eq!(hsv.to_hsv().unwrap(), (0.1, 0.2, 0.3));
    }

    #[test]
    #[should_panic]
    fn to_wrong_standered() {
        let cymk = Color::new_cmyk(0.1, 0.2, 0.3, 0.4).unwrap();
        let rgb = Color::new_rgb(10.2, 20.3, 30.4).unwrap();
        let hsl = Color::new_hsl(0.1, 0.2, 0.3).unwrap();
        let hsv = Color::new_hsv(0.1, 0.2, 0.3).unwrap();

        cymk.to_rgb().unwrap();
        cymk.to_hsl().unwrap();
        cymk.to_hsv().unwrap();

        rgb.to_cmyk().unwrap();
        rgb.to_hsl().unwrap();
        rgb.to_hsv().unwrap();

        hsl.to_rgb().unwrap();
        hsl.to_cmyk().unwrap();
        hsl.to_hsv().unwrap();

        hsv.to_rgb().unwrap();
        hsv.to_hsl().unwrap();
        hsv.to_cmyk().unwrap();
    }

    #[test]
    fn get_internel_color() {
        let cymk = Color::new_cmyk(0.1, 0.2, 0.3, 0.4).unwrap();
        let rgb = Color::new_rgb(10.2, 20.3, 30.4).unwrap();
        let hsl = Color::new_hsl(0.1, 0.2, 0.3).unwrap();
        let hsv = Color::new_hsv(0.1, 0.2, 0.3).unwrap();

        assert_eq!(cymk.get_internel_color(), (0.1, 0.2, 0.3, Some(0.4)));
        assert_eq!(rgb.get_internel_color(), (10.2, 20.3, 30.4, None));
        assert_eq!(hsl.get_internel_color(), (0.1, 0.2, 0.3, None));
        assert_eq!(hsv.get_internel_color(), (0.1, 0.2, 0.3, None));
    }

    #[test]
    #[should_panic]
    fn get_internel_color_4th_element() {
        let cymk = Color::new_cmyk(0.1, 0.2, 0.3, 0.4)
            .unwrap()
            .get_internel_color();
        let rgb = Color::new_rgb(10.2, 20.3, 30.4)
            .unwrap()
            .get_internel_color();
        let hsl = Color::new_hsl(0.1, 0.2, 0.3).unwrap().get_internel_color();
        let hsv = Color::new_hsv(0.1, 0.2, 0.3).unwrap().get_internel_color();

        if cymk.3.is_none() {
            panic!();
        }

        rgb.3.unwrap();
        hsl.3.unwrap();
        hsv.3.unwrap();
    }

    #[test]
    fn as_standered_rgb() {
        let cymk = Color::new_cmyk(0.1, 0.2, 0.3, 0.4).unwrap();
        let rgb = Color::new_rgb(10.2, 20.3, 30.4).unwrap();
        let hsl = Color::new_hsl(0.1, 0.2, 0.3).unwrap();
        let hsv = Color::new_hsv(0.1, 0.2, 0.3).unwrap();
        let standered = ColorStandered::Rgb;

        cymk.as_standered(standered);
        rgb.as_standered(standered);
        hsl.as_standered(standered);
        hsv.as_standered(standered);

        assert_eq!(
            cymk.to_rgb().unwrap(),
            to_rgb::cmyk_to_rgb(0.1, 0.2, 0.3, 0.4)
        );
        assert_eq!(rgb.to_rgb().unwrap(), (10.2, 20.3, 30.4));
        assert_eq!(hsl.to_rgb().unwrap(), to_rgb::hsl_to_rgb(0.1, 0.2, 0.3));
        assert_eq!(hsv.to_rgb().unwrap(), to_rgb::hsv_to_rgb(0.1, 0.2, 0.3));
    }

    #[test]
    fn as_standered_cmyk() {
        let cymk = Color::new_cmyk(0.1, 0.2, 0.3, 0.4).unwrap();
        let rgb = Color::new_rgb(10.2, 20.3, 30.4).unwrap();
        let hsl = Color::new_hsl(0.1, 0.2, 0.3).unwrap();
        let hsv = Color::new_hsv(0.1, 0.2, 0.3).unwrap();
        let standered = ColorStandered::Cmyk;

        cymk.as_standered(standered);
        rgb.as_standered(standered);
        hsl.as_standered(standered);
        hsv.as_standered(standered);

        assert_eq!(cymk.to_cmyk().unwrap(), (0.1, 0.2, 0.3, 0.4));
        assert_eq!(
            rgb.to_cmyk().unwrap(),
            from_rgb::rgb_to_cmyk(10.2, 20.3, 30.4)
        );
        assert_eq!(
            hsl.to_cmyk().unwrap(),
            from_rgb::rgb_to_cmyk(
                to_rgb::hsl_to_rgb(0.1, 0.2, 0.3).0,
                to_rgb::hsl_to_rgb(0.1, 0.2, 0.3).1,
                to_rgb::hsl_to_rgb(0.1, 0.2, 0.3).2
            )
        );
        assert_eq!(
            hsv.to_cmyk().unwrap(),
            from_rgb::rgb_to_cmyk(
                to_rgb::hsv_to_rgb(0.1, 0.2, 0.3).0,
                to_rgb::hsv_to_rgb(0.1, 0.2, 0.3).1,
                to_rgb::hsv_to_rgb(0.1, 0.2, 0.3).2
            )
        );
    }

    #[test]
    fn as_standered_hsl() {
        let cymk = Color::new_cmyk(0.1, 0.2, 0.3, 0.4).unwrap();
        let rgb = Color::new_rgb(10.2, 20.3, 30.4).unwrap();
        let hsl = Color::new_hsl(0.1, 0.2, 0.3).unwrap();
        let hsv = Color::new_hsv(0.1, 0.2, 0.3).unwrap();
        let standered = ColorStandered::Hsl;

        cymk.as_standered(standered);
        rgb.as_standered(standered);
        hsl.as_standered(standered);
        hsv.as_standered(standered);

        assert_eq!(
            cymk.to_hsl().unwrap(),
            from_rgb::rgb_to_hsl(
                to_rgb::cmyk_to_rgb(0.1, 0.2, 0.3, 0.4).0,
                to_rgb::cmyk_to_rgb(0.1, 0.2, 0.3, 0.4).1,
                to_rgb::cmyk_to_rgb(0.1, 0.2, 0.3, 0.4).2
            )
        );
        assert_eq!(
            rgb.to_hsl().unwrap(),
            from_rgb::rgb_to_hsl(10.2, 20.3, 30.4)
        );
        assert_eq!(hsl.to_hsl().unwrap(), (0.1, 0.2, 0.3));
        assert_eq!(
            hsv.to_hsl().unwrap(),
            from_rgb::rgb_to_hsl(
                to_rgb::hsv_to_rgb(0.1, 0.2, 0.3).0,
                to_rgb::hsv_to_rgb(0.1, 0.2, 0.3).1,
                to_rgb::hsv_to_rgb(0.1, 0.2, 0.3).2
            )
        );
    }

    #[test]
    fn as_standered_hsv() {
        let cymk = Color::new_cmyk(0.1, 0.2, 0.3, 0.4).unwrap();
        let rgb = Color::new_rgb(10.2, 20.3, 30.4).unwrap();
        let hsl = Color::new_hsl(0.1, 0.2, 0.3).unwrap();
        let hsv = Color::new_hsv(0.1, 0.2, 0.3).unwrap();
        let standered = ColorStandered::Hsv;

        cymk.as_standered(standered);
        rgb.as_standered(standered);
        hsl.as_standered(standered);
        hsv.as_standered(standered);

        assert_eq!(
            cymk.to_hsv().unwrap(),
            from_rgb::rgb_to_hsv(
                to_rgb::cmyk_to_rgb(0.1, 0.2, 0.3, 0.4).0,
                to_rgb::cmyk_to_rgb(0.1, 0.2, 0.3, 0.4).1,
                to_rgb::cmyk_to_rgb(0.1, 0.2, 0.3, 0.4).2
            )
        );
        assert_eq!(
            rgb.to_hsv().unwrap(),
            from_rgb::rgb_to_hsv(10.2, 20.3, 30.4)
        );
        assert_eq!(
            hsl.to_hsv().unwrap(),
            from_rgb::rgb_to_hsv(
                to_rgb::hsl_to_rgb(0.1, 0.2, 0.3).0,
                to_rgb::hsl_to_rgb(0.1, 0.2, 0.3).1,
                to_rgb::hsl_to_rgb(0.1, 0.2, 0.3).2
            )
        );
        assert_eq!(hsv.to_hsv().unwrap(), (0.1, 0.2, 0.3));
    }

    #[test]
    fn into_standered_rgb() {
        let cymk = Color::new_cmyk(0.1, 0.2, 0.3, 0.4).unwrap();
        let rgb = Color::new_rgb(10.2, 20.3, 30.4).unwrap();
        let hsl = Color::new_hsl(0.1, 0.2, 0.3).unwrap();
        let hsv = Color::new_hsv(0.1, 0.2, 0.3).unwrap();
        let standered = ColorStandered::Rgb;

        let cmyk = cymk.into_standered(standered);
        let rgb = rgb.into_standered(standered);
        let hsl = hsl.into_standered(standered);
        let hsv = hsv.into_standered(standered);

        assert_eq!(
            (cmyk.0, cmyk.1, cmyk.2),
            to_rgb::cmyk_to_rgb(0.1, 0.2, 0.3, 0.4)
        );
        assert_eq!((rgb.0, rgb.1, rgb.2), (10.2, 20.3, 30.4));
        assert_eq!((hsl.0, hsl.1, hsl.2), to_rgb::hsl_to_rgb(0.1, 0.2, 0.3));
        assert_eq!((hsv.0, hsv.1, hsv.2), to_rgb::hsv_to_rgb(0.1, 0.2, 0.3));
    }

    #[test]
    fn into_standered_cmyk() {
        let cymk = Color::new_cmyk(0.1, 0.2, 0.3, 0.4).unwrap();
        let rgb = Color::new_rgb(10.2, 20.3, 30.4).unwrap();
        let hsl = Color::new_hsl(0.1, 0.2, 0.3).unwrap();
        let hsv = Color::new_hsv(0.1, 0.2, 0.3).unwrap();
        let standered = ColorStandered::Cmyk;

        let cmyk = cymk.into_standered(standered);
        let rgb = rgb.into_standered(standered);
        let hsl = hsl.into_standered(standered);
        let hsv = hsv.into_standered(standered);

        assert_eq!(cmyk, (0.1, 0.2, 0.3, Some(0.4)));
        assert_eq!(rgb, into(from_rgb::rgb_to_cmyk(10.2, 20.3, 30.4)));
        assert_eq!(
            hsl,
            into(from_rgb::rgb_to_cmyk(
                to_rgb::hsl_to_rgb(0.1, 0.2, 0.3).0,
                to_rgb::hsl_to_rgb(0.1, 0.2, 0.3).1,
                to_rgb::hsl_to_rgb(0.1, 0.2, 0.3).2
            ))
        );
        assert_eq!(
            hsv,
            into(from_rgb::rgb_to_cmyk(
                to_rgb::hsv_to_rgb(0.1, 0.2, 0.3).0,
                to_rgb::hsv_to_rgb(0.1, 0.2, 0.3).1,
                to_rgb::hsv_to_rgb(0.1, 0.2, 0.3).2
            ))
        );
    }

    #[test]
    fn into_standered_hsl() {
        let cymk = Color::new_cmyk(0.1, 0.2, 0.3, 0.4).unwrap();
        let rgb = Color::new_rgb(10.2, 20.3, 30.4).unwrap();
        let hsl = Color::new_hsl(0.1, 0.2, 0.3).unwrap();
        let hsv = Color::new_hsv(0.1, 0.2, 0.3).unwrap();
        let standered = ColorStandered::Hsl;

        let cmyk = cymk.into_standered(standered);
        let rgb = rgb.into_standered(standered);
        let hsl = hsl.into_standered(standered);
        let hsv = hsv.into_standered(standered);

        assert_eq!(
            (cmyk.0, cmyk.1, cmyk.2),
            from_rgb::rgb_to_hsl(
                to_rgb::cmyk_to_rgb(0.1, 0.2, 0.3, 0.4).0,
                to_rgb::cmyk_to_rgb(0.1, 0.2, 0.3, 0.4).1,
                to_rgb::cmyk_to_rgb(0.1, 0.2, 0.3, 0.4).2
            )
        );
        assert_eq!(
            (rgb.0, rgb.1, rgb.2),
            from_rgb::rgb_to_hsl(10.2, 20.3, 30.4)
        );
        assert_eq!((hsl.0, hsl.1, hsl.2), (0.1, 0.2, 0.3));
        assert_eq!(
            (hsv.0, hsv.1, hsv.2),
            from_rgb::rgb_to_hsl(
                to_rgb::hsv_to_rgb(0.1, 0.2, 0.3).0,
                to_rgb::hsv_to_rgb(0.1, 0.2, 0.3).1,
                to_rgb::hsv_to_rgb(0.1, 0.2, 0.3).2
            )
        );
    }

    #[test]
    fn into_standered_hsv() {
        let cymk = Color::new_cmyk(0.1, 0.2, 0.3, 0.4).unwrap();
        let rgb = Color::new_rgb(10.2, 20.3, 30.4).unwrap();
        let hsl = Color::new_hsl(0.1, 0.2, 0.3).unwrap();
        let hsv = Color::new_hsv(0.1, 0.2, 0.3).unwrap();
        let standered = ColorStandered::Hsv;

        let cmyk = cymk.into_standered(standered);
        let rgb = rgb.into_standered(standered);
        let hsl = hsl.into_standered(standered);
        let hsv = hsv.into_standered(standered);

        assert_eq!(
            (cmyk.0, cmyk.1, cmyk.2),
            from_rgb::rgb_to_hsv(
                to_rgb::cmyk_to_rgb(0.1, 0.2, 0.3, 0.4).0,
                to_rgb::cmyk_to_rgb(0.1, 0.2, 0.3, 0.4).1,
                to_rgb::cmyk_to_rgb(0.1, 0.2, 0.3, 0.4).2
            )
        );
        assert_eq!(
            (rgb.0, rgb.1, rgb.2),
            from_rgb::rgb_to_hsv(10.2, 20.3, 30.4)
        );
        assert_eq!(
            (hsl.0, hsl.1, hsl.2),
            from_rgb::rgb_to_hsv(
                to_rgb::hsl_to_rgb(0.1, 0.2, 0.3).0,
                to_rgb::hsl_to_rgb(0.1, 0.2, 0.3).1,
                to_rgb::hsl_to_rgb(0.1, 0.2, 0.3).2
            )
        );
        assert_eq!((hsv.0, hsv.1, hsv.2), (0.1, 0.2, 0.3));
    }

    // TODO: Fix this test

    // #[test]
    // fn as_rgb_with_alpha() {
    //     let fore = Color::new_rgb(10.2,20.3,30.4).unwrap();
    //     let back = Color::new_rgb(40.3,50.4,60.5).unwrap();

    //     let res = fore.as_rgb_with_alpha_no_handle(back, 0.1).unwrap();

    //     assert_eq!(res.get_internel_color(), Color::new(20.66, 26.215, 31.77).unwrap().get_internel_color());
    // }
}
