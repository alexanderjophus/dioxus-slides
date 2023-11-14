use dioxus::events::KeyboardEvent;
use dioxus::html::input_data::keyboard_types::Key;
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

    #[props(default = true)]
    show_slide_progress_bar: bool,

    #[props(default = true)]
    enable_keyboard_navigation: bool,
}

pub fn SlideContainer<'a, S: Slidable + Clone + Default>(
    cx: Scope<'a, SlideContainerProps<'a, S>>,
) -> Element
where
    <S as FromStr>::Err: std::fmt::Display,
    <S as FromStr>::Err: std::fmt::Debug,
{
    let deck = use_state::<S>(cx, || S::default());
    let prev = deck.prev();
    let next = deck.next();

    let onkeydown = move |event: KeyboardEvent| {
        if cx.props.enable_keyboard_navigation {
            let a = match event.key() {
                Key::ArrowRight => deck.next(),
                Key::ArrowLeft => deck.prev(),
                _ => None,
            };
            if let Some(a) = a {
                deck.set(a)
            }
        }
    };

    cx.render(rsx! {
        div {
            tabindex: "0",
            onkeydown: onkeydown,
            style: "position: relative; min-height: 99vh; min-width: 99vw;",
            div {
                style: "z-index: 0; position: absolute; background-color: {cx.props.background_colour}; width: {cx.props.width}; height: {cx.props.height};",
                deck.render(cx)
            }
            if let Some(prev) = prev.clone() {
                render! {
                    div {
                        style: "z-index: 10; height: 100%; width: 20%; position: absolute; top: 0; left: 0;",
                        onclick: move |_| {
                            deck.set(prev.clone())
                        }
                    }
                }
            }
            if let Some(next) = next.clone() {
                render! {
                    div {
                        style: "z-index: 10; height: 100%; width: 20%; position: absolute; top: 0; left: 80%;",
                        onclick: move |_| {
                            deck.set(next.clone())
                        }
                    }
                }
            }
            if cx.props.show_slide_progress_bar {
                render! {
                    progress {
                        style: "z-index: 10; position: absolute; top: 0; left: 0; width: 100%;",
                        max: "{deck.number_of_slides().to_string()}",
                        value: "{deck.slide_number().to_string()}",
                    }
                }
            },
            if cx.props.show_slide_no {
                render! {
                    div {
                        style: "z-index: 10; position: absolute; bottom: 5%; right: 5%;",
                        "Slide: {deck.slide_number().to_string()} of {deck.number_of_slides().to_string()}"
                    }
                }
            }
        }
    })
}
