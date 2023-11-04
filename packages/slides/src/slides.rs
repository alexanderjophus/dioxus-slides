use dioxus::prelude::*;
use std::{fmt::Display, marker::PhantomData, str::FromStr};

/// This trait can be derived using the `#[derive(Slidable)]` macro.
pub trait Slidable: FromStr + Display + Clone + 'static {
    fn render<'a>(&self, cx: &'a ScopeState) -> Element<'a>;
}

#[derive(Props, PartialEq)]
pub struct SliderProps<R: Slidable + Clone> {
    #[props(default, into)]
    config: SliderConfig,
    #[props(default)]
    phantom: PhantomData<R>,
}

pub fn Slider<R: Slidable + Clone>(cx: Scope<SliderProps<R>>) -> Element
where
    <R as FromStr>::Err: std::fmt::Display,
{
    render!(
        div {
            h1 { "Hello, world!" }
            p { "This is a slide." }
        }
    )
    // let slide = cx.state::<R>().unwrap_or_default();
    // slide.render(cx)
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
