pub mod attributes;
pub mod condition;
mod context;
pub mod fonts;
pub mod header;
mod id;
mod size;
mod spacing;
mod style;
mod tag;

pub use context::Context;
pub use header::Header;
pub use id::generate as generate_id;
pub use size::Size;
pub use spacing::Spacing;
pub use style::Style;
pub use tag::Tag;

use std::cmp::Ordering;

pub fn sort_by_key<'r, 's>(a: &'r (&String, &String), b: &'s (&String, &String)) -> Ordering {
    a.0.partial_cmp(&b.0).unwrap()
}
