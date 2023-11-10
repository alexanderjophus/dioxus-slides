#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_slides::prelude::*;

fn main() {
    dioxus_desktop::launch(app);
}

#[derive(Slidable, Clone, Default)]
enum Slides {
    #[default]
    Intro,
    Second,
    Final,
}

fn app(cx: Scope) -> Element {
    render! {
        SlideContainer::<Slides>{
            width: "100%",
            height: "100%",
            background_colour: "#eee",
            show_slide_no: true,
        }
    }
}

fn Intro(cx: Scope) -> Element {
    cx.render(rsx!(Slide::<Slides> {
        content: render! {
            div {
                h1 { "Hello, world!" }
                p { "This is the first slide." }
            }
        },
    }))
}

fn Second(cx: Scope) -> Element {
    cx.render(rsx!(Slide::<Slides> {
        content: render! {
            div {
                h1 { "Hello, world!" }
                p { "This is another slide." }
            }
        },
    }))
}

fn Final(cx: Scope) -> Element {
    cx.render(rsx!(Slide::<Slides> {
        content: render! {
            div {
                h1 { "Hello, world!" }
                p { "This is the final slide." }
            }
        },
    }))
}
