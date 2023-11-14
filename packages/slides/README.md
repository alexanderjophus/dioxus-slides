# Dioxus Slides

Dioxus Slides is a slides component library for Dioxus, a reactive UI library for Rust. It provides a way to create slide presentations using Dioxus and Rust.

## Features

- Slide navigation: Navigate through slides using the `next` and `prev` methods provided by the [`Slidable`](packages/slides-macro/src/lib.rs#L5) trait.
- Slide rendering: Render slides using the `render` method provided by the [`Slidable`](packages/slides-macro/src/lib.rs#L6) trait.
- Slide numbering: Get the current slide number and the total number of slides.

## Usage

First, add `dioxus-slides` to your `Cargo.toml`:

```toml
[dependencies]
dioxus = "0.4.0"
dioxus-slides = "0.1"
```

Then, define your slides as an enum and derive the Slidable trait:

```rust
#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_slides::prelude::*;

#[derive(Slidable, Clone, Default)]
enum Slides {
    #[default]
    Intro,
    Second,
    Final,
}
```

Each variant of the enum represents a slide. You can then define a function for each slide that returns an Element:

```rust
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

// Define functions for the other slides...
```

Finally, you can use the SlideContainer component to render your slides:

```rust
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
```

License

This project is licensed under either of

Apache License, Version 2.0, [LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0) [MIT license](http://opensource.org/licenses/MIT) at your option.


Please note that this README assumes that the `LICENSE-APACHE` and `LICENSE-MIT` files exist in the root directory of the project.

