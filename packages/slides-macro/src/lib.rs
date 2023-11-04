use proc_macro::TokenStream;
use quote::quote;
use syn::{
    __private::TokenStream2,
    {parse_macro_input, Ident},
};

#[proc_macro_derive(Slidable, attributes(slide))]
pub fn slidable(input: TokenStream) -> TokenStream {
    let item_enum = parse_macro_input!(input as syn::ItemEnum);

    let slides_enum = match SlideEnum::parse(item_enum) {
        Ok(slides_enum) => slides_enum,
        Err(err) => return err.to_compile_error().into(),
    };

    let display_impl = slides_enum.impl_display();
    let parse_impl = slides_enum.parse_impl();
    let slidable_impl = slides_enum.slidable_impl();

    (quote! {
        #display_impl

        #parse_impl

        #slidable_impl
    })
    .into()
}

struct SlideEnum {
    name: Ident,
    slides: Vec<Slide>,
}

impl SlideEnum {
    fn parse(data: syn::ItemEnum) -> syn::Result<Self> {
        let name = &data.ident;
        let mut slides = Vec::new();

        for variant in data.variants {
            let slide_name = variant.ident;

            slides.push(Slide { slide_name });
        }

        let myself = Self {
            name: name.clone(),
            slides,
        };

        Ok(myself)
    }

    fn impl_display(&self) -> TokenStream2 {
        let name = &self.name;

        let mut display_match = Vec::new();

        for slide in &self.slides {
            display_match.push(slide.display_match());
        }

        quote! {
            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #(#display_match)*
                    }
                    Ok(())
                }
            }
        }
    }

    fn parse_impl(&self) -> TokenStream2 {
        let name = &self.name;

        let mut display_match = Vec::new();

        for slide in &self.slides {
            display_match.push(slide.match_from_str());
        }

        quote! {
            impl std::str::FromStr for #name {
                type Err = String;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        #(#display_match)*
                        _ => Err(format!("Unknown slide: {}", s))
                    }
                }
            }
        }
    }

    fn slidable_impl(&self) -> TokenStream2 {
        let name = &self.name;

        let mut matches = Vec::new();
        // Collect all routes matches
        for slide in &self.slides {
            matches.push(slide.slidable_match());
        }

        quote! {
            impl dioxus_slides::Slidable for #name where Self: Clone {
                fn render<'a>(&self, cx: &'a dioxus::prelude::ScopeState) -> dioxus::prelude::Element<'a> {
                    match self {
                        #(#matches)*
                        _ => None
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct Slide {
    pub slide_name: Ident,
}

impl Slide {
    fn match_from_str(&self) -> TokenStream2 {
        let slide_name = &self.slide_name;

        quote! {
            stringify!(#slide_name) => Ok(Self::#slide_name {}),
        }
    }

    fn display_match(&self) -> TokenStream2 {
        let slide_name = &self.slide_name;

        quote! {
            Self::#slide_name {} => write!(f, "{}", stringify!(#slide_name))?,
        }
    }

    fn slidable_match(&self) -> TokenStream2 {
        let slide_name = &self.slide_name;

        quote! {
            Self::#slide_name {} => {
                render! {
                    #slide_name {}
                }
            }
        }
    }
}