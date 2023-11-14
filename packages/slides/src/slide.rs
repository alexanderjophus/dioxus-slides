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
    let deck = use_shared_state::<S>(cx).expect("Failed to get shared state");

    let prev = deck.read().prev();
    let next = deck.read().next();

    cx.render(rsx! {
        div {
            style: "position: relative; min-height: 99vh; min-width: 99vw;",
            div {
                style: "z-index: 0; height: 100%; width: 100%; position: absolute; top: 0; left: 0;",
                cx.props.content.clone()
            }
            if let Some(prev) = prev.clone() {
                render! {
                    div {
                        // show on the left side of the screen for 20% of the screen
                        style: "z-index: 10; height: 100%; width: 20%; position: absolute; top: 0; left: 0;",
                        onclick: move |_| {
                            let mut deck = deck.write();
                            *deck = prev.clone();
                        }
                    }
                }
            }
            if let Some(next) = next.clone() {
                render! {
                    div {
                        style: "z-index: 10; height: 100%; width: 20%; position: absolute; top: 0; left: 80%;",
                        onclick: move |_| {
                            let mut deck = deck.write();
                            *deck = next.clone();
                        }
                    }
                }
            }
        }
    })
}
