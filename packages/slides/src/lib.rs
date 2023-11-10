#![allow(non_snake_case)]

mod slide;
mod slide_container;
pub use slide::*;
pub use slide_container::*;

pub mod prelude {
    pub use crate::*;
    pub use dioxus_slides_macro::Slidable;
}
