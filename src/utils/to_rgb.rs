pub fn hsl_to_rgb(hue: f64, sateration: f64, lightness: f64) -> (f64, f64, f64) {
    let c = (1.-(2.*lightness-1.).abs())*sateration;
    let x = c*(1.-((hue/60.)%2.-1.).abs());
    let m = lightness-c/2.;
    
    let (r, g, b) = 
    if (0. ..60.).contains(&hue)         {(c, x, 0.)}   
    else if (60. ..120.).contains(&hue)  {(x, c, 0.)}  
    else if (120. ..180.).contains(&hue) {(0., c, x)} 
    else if (180. ..240.).contains(&hue) {(0., x, c)} 
    else if (240. ..300.).contains(&hue) {(x, 0., c)} 
    else if (300. ..360.).contains(&hue) {(c, 0., x)} 
    else{panic!("invalid hue")};

    ((r+m)*255., (g+m)*255., (b+m)*255.,)
}

pub fn hsv_to_rgb(hue: f64, sateration: f64, value: f64) -> (f64, f64, f64) {
    let c = value*sateration;
    let x = c*(1.-((hue/60.)%2.-1.).abs());
    let m = value-c;
    
    let (r, g, b) = 
    if (0. ..60.).contains(&hue)         {(c, x, 0.)} 
    else if (60. ..120.).contains(&hue)  {(x, c, 0.)} 
    else if (120. ..180.).contains(&hue) {(0., c, x)}
    else if (180. ..240.).contains(&hue) {(0., x, c)} 
    else if (240. ..300.).contains(&hue) {(x, 0., c)} 
    else if (300. ..360.).contains(&hue) {(c, 0., x)} 
    else {panic!("invalid hue")};

    ((r+m)*255., (g+m)*255., (b+m)*255.,)
}

pub fn cymk_to_rgb(cyan: f64, magenta: f64, yellow: f64, black: f64) -> (f64, f64, f64) {
    (255.*(1.-cyan)*(1.-black), 255.*(1.-magenta)*(1.-black), 255.*(1.-yellow)*(1.-black))
}