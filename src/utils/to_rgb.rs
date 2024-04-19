pub fn hsl_to_rgb(hue: f64, sateration: f64, lightness: f64) -> (f64, f64, f64) {
    let c = (1. - (2. * lightness - 1.).abs()) * sateration;
    let x = c * (1. - ((hue / 60.) % 2. - 1.).abs());
    let m = lightness - c / 2.;

    let (r, g, b) = if (0. ..60.).contains(&hue) {
        (c, x, 0.)
    } else if (60. ..120.).contains(&hue) {
        (x, c, 0.)
    } else if (120. ..180.).contains(&hue) {
        (0., c, x)
    } else if (180. ..240.).contains(&hue) {
        (0., x, c)
    } else if (240. ..300.).contains(&hue) {
        (x, 0., c)
    } else if (300. ..360.).contains(&hue) {
        (c, 0., x)
    } else {
        panic!("invalid hue")
    };

    ((r + m) * 255., (g + m) * 255., (b + m) * 255.)
}

pub fn hsv_to_rgb(hue: f64, sateration: f64, value: f64) -> (f64, f64, f64) {
    let c = value * sateration;
    let x = c * (1. - ((hue / 60.) % 2. - 1.).abs());
    let m = value - c;

    let (r, g, b) = if (0. ..60.).contains(&hue) {
        (c, x, 0.)
    } else if (60. ..120.).contains(&hue) {
        (x, c, 0.)
    } else if (120. ..180.).contains(&hue) {
        (0., c, x)
    } else if (180. ..240.).contains(&hue) {
        (0., x, c)
    } else if (240. ..300.).contains(&hue) {
        (x, 0., c)
    } else if (300. ..360.).contains(&hue) {
        (c, 0., x)
    } else {
        panic!("invalid hue")
    };

    ((r + m) * 255., (g + m) * 255., (b + m) * 255.)
}

pub fn cmyk_to_rgb(cyan: f64, magenta: f64, yellow: f64, black: f64) -> (f64, f64, f64) {
    (
        255. * (1. - cyan) * (1. - black),
        255. * (1. - magenta) * (1. - black),
        255. * (1. - yellow) * (1. - black),
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::from_rgb::*;
    use rand::prelude::*;
    use std::{fs::File, io::Write};
    use colors_transform::{Rgb, Color, Hsl};

    #[test]
    fn test_hsl() {
        let mut rng = thread_rng();
        let epsilon = 0.001;

        for i in 0..100000 {
            let start: (f32, f32, f32) = (
                rng.gen_range(0. ..=360.),
                rng.gen_range(0. ..=1.),
                rng.gen_range(0. ..=1.),
            );
            let data = hsl_to_rgb(start.0 as f64, start.1 as f64, start.2 as f64);
            let rgb2 = Hsl::from(start.0, start.1, start.2).to_rgb().as_tuple();
            let rgb2 = (rgb2.0 as f64, rgb2.1 as f64, rgb2.2 as f64);

            assert!((data.0-rgb2.0/255.).abs() <= epsilon, "({}, {}, {}) became ({}, {}, {}) on the {}th iteration",
            start.0,
            start.1,
            start.2,
            rgb2.0/255.,
            rgb2.1/255.,
            rgb2.2/255.,
            i);
            assert!((data.1-rgb2.1/255.).abs() <= epsilon, "({}, {}, {}) became ({}, {}, {}) on the {}th iteration",
            start.0,
            start.1,
            start.2,
            rgb2.0/255.,
            rgb2.1/255.,
            rgb2.2/255.,
            i);
            assert!((data.2-rgb2.2/255.).abs() <= epsilon, "({}, {}, {}) became ({}, {}, {}) on the {}th iteration",
            start.0,
            start.1,
            start.2,
            rgb2.0/255.,
            rgb2.1/255.,
            rgb2.2/255.,
            i);
        }
    }
    

    #[test]
    fn test_hsv() {
        let mut rng = thread_rng();

        for i in 0..100000 {
            let start: (f64, f64, f64) = (
                rng.gen_range(0. ..=360.),
                rng.gen_range(0. ..=1.),
                rng.gen_range(0. ..=1.),
            );
            let data = hsv_to_rgb(start.0, start.1, start.2);
            let data = rgb_to_hsv(data.0, data.1, data.2);
            assert!(
                (start.0 - data.0).abs() <= 0.0000001,
                "({}, {}, {}) became ({}, {}, {}) on the {}th iteration",
                start.0,
                start.1,
                start.2,
                data.0,
                data.1,
                data.2,
                i
            );
            assert!(
                (start.1 - data.1).abs() <= 0.0000001,
                "({}, {}, {}) became ({}, {}, {}) on the {}th iteration",
                start.0,
                start.1,
                start.2,
                data.0,
                data.1,
                data.2,
                i
            );
            assert!(
                (start.2 - data.2).abs() <= 0.0000001,
                "({}, {}, {}) became ({}, {}, {}) on the {}th iteration",
                start.0,
                start.1,
                start.2,
                data.0,
                data.1,
                data.2,
                i
            );
        }
    }

    /*  #[test]
    // fn test_cmyk() {
    //     let mut rng = thread_rng();

    //     for i in 0..100000 {
    //         let start = (
    //             rng.gen_range(0. ..=1.),
    //             rng.gen_range(0. ..=1.),
    //             rng.gen_range(0. ..=1.),
    //             rng.gen_range(0. ..=1.),
    //         );

    //         let data1 = cmyk_to_rgb(start.0, start.1, start.2, start.3);
    //         let data = rgb_to_cmyk(data1.0, data1.1, data1.2);
    //         let data = cmyk_to_rgb(data1.0, data1.1, data1.2, data.3);

    //         assert!(
    //             (data1.0 - data.0).abs() <= 0.0000001,
    //             "({}, {}, {}) became ({}, {}, {}) on the {}th iteration",
    //             data1.0,
    //             data1.1,
    //             data1.2,
    //             data.0,
    //             data.1,
    //             data.2,
    //             i
    //         );
    //         assert!(
    //             (data1.1 - data.1).abs() <= 0.0000001,
    //             "({}, {}, {}) became ({}, {}, {}) on the {}th iteration",
    //             data1.0,
    //             data1.1,
    //             data1.2,
    //             data.0,
    //             data.1,
    //             data.2,
    //             i
    //         );
    //         assert!(
    //             (data1.2 - data.2).abs() <= 0.0000001,
    //             "({}, {}, {}) became ({}, {}, {}) on the {}th iteration",
    //             data1.0,
    //             data1.1,
    //             data1.2,
    //             data.0,
    //             data.1,
    //             data.2,
    //             i
    //         );
    //     }
    // }*/
}

