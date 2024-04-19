#![allow(clippy::upper_case_acronyms)]
use crate::color_print::NumType;

type StructHandle = wai_bindgen_rust::Handle<crate::Color>;

pub type Wrapper<T> = std::cell::Cell<Option<T>>;
pub type NewColorResult = Result<StructHandle, crate::color_print::Exeptions>;

pub type RGB = (NumType, NumType, NumType);
pub type HSL = (NumType, NumType, NumType);
pub type HSV = (NumType, NumType, NumType);
pub type CMYK = (NumType, NumType, NumType, NumType);
