wai_bindgen_rust::export!("color_print.wai");

struct ColorPrint;

impl crate::color_print::ColorPrint for ColorPrint {
    fn test() -> () {
        println!("this is cool")
    }
}