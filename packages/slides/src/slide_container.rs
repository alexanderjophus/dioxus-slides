use dioxus::prelude::*;
use std::{marker::PhantomData, str::FromStr};

use crate::Slidable;

#[derive(Props)]
pub struct SlideContainerProps<'a, S: Slidable + Clone> {
    #[props(default)]
    phantom: PhantomData<S>,

    #[props(default = "100%")]
    width: &'a str,
    #[props(default = "100%")]
    height: &'a str,

    #[props(default = "#fff")]
    background_colour: &'a str,

    #[props(default = false)]
    show_slide_no: bool,
}

pub fn SlideContainer<'a, S: Slidable + Clone + Default>(
    cx: Scope<'a, SlideContainerProps<'a, S>>,
) -> Element
where
    <S as FromStr>::Err: std::fmt::Display,
    <S as FromStr>::Err: std::fmt::Debug,
{
    use_shared_state_provider(cx, || S::default());
    let deck = use_shared_state::<S>(cx).expect("Failed to get shared state");

    cx.render(rsx! {
        div {
            style: "position: relative; min-height: 100vh; min-width: 100vw;",
            div {
                style: "z-index: 0; position: absolute; background-color: {cx.props.background_colour}; width: {cx.props.width}; height: {cx.props.height};",
                deck.read().render(cx)
            }
            if cx.props.show_slide_no {
                render! {
                    div {
                        style: "z-index: 10; position: absolute; bottom: 5%; right: 5%;",
                        "Slide: {deck.read().slide_number().to_string()} of {deck.read().number_of_slides().to_string()}"
                    }
                }
            }
        }
    })
}
