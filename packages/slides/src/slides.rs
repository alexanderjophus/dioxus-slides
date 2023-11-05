use dioxus::prelude::*;
use std::{fmt::Display, marker::PhantomData, str::FromStr};

/// This trait can be derived using the `#[derive(Slidable)]` macro.
pub trait Slidable: FromStr + Display + Clone + 'static {
    fn render<'a>(&self, cx: &'a ScopeState) -> Element<'a>;
}

#[derive(Props, PartialEq)]
pub struct SliderProps<R: Slidable + Clone> {
    first_slide: String,
    #[props(default)]
    phantom: PhantomData<R>,
}

pub fn Slider<R: Slidable + Clone>(cx: Scope<SliderProps<R>>) -> Element
where
    <R as FromStr>::Err: std::fmt::Display,
    <R as FromStr>::Err: std::fmt::Debug,
{
    let slide = &R::from_str(&cx.props.first_slide).expect("Failed to parse slide name");
    slide.render(cx)
}

#[derive(Props)]
pub struct SlideProps<'a> {
    content: Element<'a>,
    next: Option<&'a str>,
}

pub fn Slide<'a>(cx: Scope<'a, SlideProps<'a>>) -> Element {
    cx.render(rsx! {
        &cx.props.content
        if let Some(next) = cx.props.next {
            render! {
                a {
                    href: next,
                    "Next"
                }
            }
        }
    })
}
