use dioxus::prelude::*;
use std::{fmt::Display, marker::PhantomData, str::FromStr};

/// This trait can be derived using the `#[derive(Slidable)]` macro.
pub trait Slidable: FromStr + Display + Clone + 'static {
    fn render<'a>(&self, cx: &'a ScopeState) -> Element<'a>;
}

#[derive(Props, PartialEq)]
pub struct SliderProps<R: Slidable + Clone> {
    #[props(default)]
    phantom: PhantomData<R>,
}

pub fn Slider<R: Slidable + Clone + Default>(cx: Scope<SliderProps<R>>) -> Element
where
    <R as FromStr>::Err: std::fmt::Display,
    <R as FromStr>::Err: std::fmt::Debug,
{
    use_shared_state_provider(cx, || R::default());
    let deck = use_shared_state::<R>(cx).expect("Failed to get shared state");
    deck.read().render(cx)
}

#[derive(Props)]
pub struct SlideProps<'a, T>
where
    T: Slidable + Clone + 'static,
{
    content: Element<'a>,
    next: Option<T>,
}

pub fn Slide<'a, T: Slidable + Clone + 'static>(cx: Scope<'a, SlideProps<'a, T>>) -> Element<'a> {
    let deck = use_shared_state::<T>(cx).expect("Failed to get shared state");

    cx.render(rsx! {
        div {
            cx.props.content.clone()
        }
        a {
            onclick: {
                let deck = deck.clone();
                move |_| {
                    let mut deck = deck.write();
                    if let Some(next) = cx.props.next.clone() {
                        *deck = next;
                    }
                }
            },
            "next"
        }
    })
}
