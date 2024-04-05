#[derive(Copy, Clone)]
pub struct RGBA {
    foreground: (f64, f64, f64),
    background: (f64, f64, f64),
    alpha: f64,
}

impl RGBA {
    pub fn new(foreground: (f64, f64, f64), background: (f64, f64, f64), alpha: f64) -> Self {
        Self {
            foreground,
            background,
            alpha
        }
    }

    pub fn change_alpha(&mut self, alpha: f64) {
        self.alpha = alpha;
    }

    pub fn change_foreground(&mut self, foreground: (f64, f64, f64)) {
        self.foreground = foreground
    }

    pub fn change_background(&mut self, background: (f64, f64, f64)) {
        self.background = background
    }
}

impl From<RGBA> for (f64, f64, f64) {
    fn from(value: RGBA) -> Self {
        let r = ((value.foreground.0*value.alpha)+value.background.0)/2.;
        let g = ((value.foreground.1*value.alpha)+value.background.1)/2.;
        let b = ((value.foreground.2*value.alpha)+value.background.2)/2.;
        (r, g, b)
    }
}