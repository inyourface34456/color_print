#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone)]
pub struct RGBA {
    pub foreground: (f64, f64, f64),
    pub background: (f64, f64, f64),
    pub alpha: f64,
}

impl RGBA {
    pub fn new(foreground: (f64, f64, f64), background: (f64, f64, f64), alpha: f64) -> Self {
        Self {
            foreground,
            background,
            alpha,
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
