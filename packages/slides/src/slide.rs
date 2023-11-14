use dioxus::prelude::*;
use std::{fmt::Display, str::FromStr};

/// This trait can be derived using the `#[derive(Slidable)]` macro.
pub trait Slidable: FromStr + Display + Clone + 'static {
    fn render<'a>(&self, cx: &'a ScopeState) -> Element<'a>;
    fn next(&self) -> Option<Self>;
    fn prev(&self) -> Option<Self>;
    fn slide_number(&self) -> usize;
    fn number_of_slides(&self) -> usize;
}

#[derive(Props)]
pub struct SlideProps<'a> {
    content: Element<'a>,
}

pub fn Slide<'a, S: Slidable + Clone + 'static>(cx: Scope<'a, SlideProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            style: "position: relative; min-height: 99vh; min-width: 99vw;",
            div {
                style: "z-index: 0; height: 100%; width: 100%; position: absolute; top: 0; left: 0;",
                cx.props.content.clone()
            }
        }
    })
}
