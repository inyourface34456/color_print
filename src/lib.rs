mod impls;
mod resourses;
mod utils;

use crate::color_print::ColorStandered;
use resourses::Color;

wai_bindgen_rust::export!("color_print.wai");

struct ColorPrint;
impl crate::color_print::ColorPrint for ColorPrint {}
