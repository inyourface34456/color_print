pub fn hsl_to_rgb(hue: f64, sateration: f64, lightness: f64) -> (f64, f64, f64) {
    let c = (1.-(2.*lightness-1.).abs())*sateration;
    let x = c*(1.-((hue/60.)%2.-1.).abs());
    let m = lightness-c/2.;
    
    let (r, g, b) = 
    if hue >= 0.   && hue < 60.  {(c, x, 0.)} else 
    if hue >= 60.  && hue < 120. {(x, c, 0.)} else 
    if hue >= 120. && hue < 180. {(0., c, x)} else 
    if hue >= 180. && hue < 240. {(0., x, c)} else 
    if hue >= 240. && hue < 300. {(x, 0., c)} else 
    if hue >= 300. && hue < 360. {(c, 0., x)} else
    {panic!("invalid hue")};

    ((r+m)*255., (g+m)*255., (b+m)*255.,)
}

pub fn hsv_to_rgb(hue: f64, sateration: f64, value: f64) -> (f64, f64, f64) {
    let c = value*sateration;
    let x = c*(1.-((hue/60.)%2.-1.).abs());
    let m = value-c;
    
    let (r, g, b) = 
    if hue >= 0.   && hue < 60.  {(c, x, 0.)} else 
    if hue >= 60.  && hue < 120. {(x, c, 0.)} else 
    if hue >= 120. && hue < 180. {(0., c, x)} else 
    if hue >= 180. && hue < 240. {(0., x, c)} else 
    if hue >= 240. && hue < 300. {(x, 0., c)} else 
    if hue >= 300. && hue < 360. {(c, 0., x)} else
    {panic!("invalid hue")};

    ((r+m)*255., (g+m)*255., (b+m)*255.,)
}

pub fn cymk_to_rgb(cyan: f64, magenta: f64, yellow: f64, black: f64) -> (f64, f64, f64) {
    (255.*(1.-cyan)*(1.-black), 255.*(1.-magenta)*(1.-black), 255.*(1.-yellow)*(1.-black))
}