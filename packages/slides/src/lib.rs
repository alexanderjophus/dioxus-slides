#![allow(non_snake_case)]

mod slides;
pub use slides::*;

pub mod prelude {
    pub use crate::*;
    pub use dioxus_slides_macro::Slidable;
}
