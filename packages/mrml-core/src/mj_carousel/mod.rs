mod children;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use crate::prelude::hash::Map;

pub use children::MJCarouselChild;

pub const NAME: &str = "mj-carousel";

#[derive(Debug, Default)]
pub struct MJCarousel {
    attributes: Map<String, String>,
    children: Vec<MJCarouselChild>,
}
