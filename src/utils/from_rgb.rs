use crate::utils::helper::{max, min};

pub fn rgb_to_hsl(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let (mut h, s, l);
    let r = r / 255.;
    let g = g / 255.;
    let b = b / 255.;

    let cmin = min(r, g, b);
    let cmax = max(r, g, b);

    let delta = cmax - cmin;

    if delta == 0. {
        h = 0.
    } else if cmax == r {
        h = (60. * (((g - b) / delta) % 6.)) % 360.;

        if h < 0. {
            h += 360.
        }
    } else if cmax == g {
        h = (60. * (((b - r) / delta) + 2.)) % 360.;
    } else {
        h = (60. * (((r - g) / delta) + 4.)) % 360.;
    }

    l = (cmin + cmax) / 2.;

    if delta == 0. {
        s = 0.
    } else {
        s = delta / (1. - 2. * l - 1.).abs()
    }

    (h, s, l)
}

pub fn rgb_to_hsv(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let (mut h, s);
    let r = r / 255.;
    let g = g / 255.;
    let b = b / 255.;

    let cmin = min(r, g, b);
    let cmax = max(r, g, b);

    let delta = cmax - cmin;

    if delta == 0. {
        h = 0.
    } else if cmax == r {
        h = (60. * (((g - b) / delta) % 6.)) % 360.;

        if h < 0. {
            h += 360.
        }
    } else if cmax == g {
        h = (60. * (((b - r) / delta) + 2.)) % 360.;
    } else {
        h = (60. * (((r - g) / delta) + 4.)) % 360.;
    }

    if cmax != 0. {
        s = delta / cmax
    } else {
        s = 0.
    }

    let delta = cmax - cmin;
    (h, s, cmax)
}

pub fn rgb_to_cmyk(r: f64, g: f64, b: f64) -> (f64, f64, f64, f64) {
    let (c, m, y, k);
    let r = r / 255.;
    let g = g / 255.;
    let b = b / 255.;

    k = 1. - max(r, g, b);
    c = (1. - r - k) / (1. - k);
    m = (1. - g - k) / (1. - k);
    y = (1. - b - k) / (1. - k);

    (c, m, y, k)
}
