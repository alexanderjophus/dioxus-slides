use dioxus::prelude::*;
use std::{fmt::Display, str::FromStr};

/// This trait can be derived using the `#[derive(Slidable)]` macro.
pub trait Slidable: FromStr + Display + Clone + 'static {
    fn render<'a>(&self, cx: &'a ScopeState) -> Element<'a>;
}

#[derive(Props, PartialEq)]
pub struct SliderProps {
    #[props(default, into)]
    config: SliderConfig,
}

pub fn Slider<R: Slidable + Clone>(cx: Scope<SliderProps>) -> Element
where
    <R as FromStr>::Err: std::fmt::Display,
{
    render! {
        div {
            h1 { "Hello, world!" }
            p { "This is a slide!" }
        }
    }
}

#[derive(Default, PartialEq)]
pub struct SliderConfig {}

#[derive(Props)]
pub struct SlideProps<'a> {
    content: Element<'a>,
}

pub fn Slide<'a>(cx: Scope<'a, SlideProps<'a>>) -> Element {
    cx.render(rsx! {
        &cx.props.content
    })
}
