#![allow(non_snake_case)]

mod slide;
pub use slide::*;

pub mod prelude {
    pub use crate::*;
    pub use dioxus_slides_macro::Slidable;
}
