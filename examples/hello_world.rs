#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_slides::prelude::*;

fn main() {
    dioxus_desktop::launch(app);
}

#[derive(Slidable, Clone)]
enum Slides {
    #[slide("intro")]
    Intro {},
    #[slide("second")]
    Second {},
    #[slide("final")]
    Final {},
}

fn app(cx: Scope) -> Element {
    render! {
        Slider::<Slides> {
            first_slide: "Intro".to_string(), // find way to define first slide
        }
    }
}

fn Intro(cx: Scope) -> Element {
    cx.render(rsx!(Slide {
        content: render! {
            div {
                h1 { "Hello, world!" }
                p { "This is the first slide." }
            }
        }
    }))
}

fn Second(cx: Scope) -> Element {
    cx.render(rsx!(Slide {
        content: render! {
            div {
                h1 { "Hello, world!" }
                p { "This is another slide." }
            }
        }
    }))
}

fn Final(cx: Scope) -> Element {
    cx.render(rsx!(Slide {
        content: render! {
            div {
                h1 { "Hello, world!" }
                p { "This is the final slide." }
            }
        }
    }))
}
