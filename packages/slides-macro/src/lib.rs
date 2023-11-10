use proc_macro::TokenStream;
use quote::{__private::Span, format_ident, quote};
use syn::{
    __private::TokenStream2,
    {parse_macro_input, Ident},
};

#[proc_macro_derive(Slidable)]
pub fn slidable(input: TokenStream) -> TokenStream {
    let item_enum = parse_macro_input!(input as syn::ItemEnum);

    let slides_enum = match SlideEnum::parse(item_enum) {
        Ok(slides_enum) => slides_enum,
        Err(err) => return err.to_compile_error().into(),
    };

    let ord_impl = slides_enum.ord_impl();
    let display_impl = slides_enum.impl_display();
    let parse_impl = slides_enum.parse_impl();
    let slidable_impl = slides_enum.slidable_impl();
    let error_type = slides_enum.error_type();

    (quote! {
        #ord_impl

        #display_impl

        #parse_impl

        #slidable_impl

        #error_type
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

    fn ord_impl(&self) -> TokenStream2 {
        let name = &self.name;

        let mut partial_eq = Vec::new();
        let mut partial_ord = Vec::new();

        for slide in &self.slides {
            let slide_name = &slide.slide_name;

            partial_eq.push(quote! {
                (#name::#slide_name {}, #name::#slide_name {}) => true,
            });
        }

        for slide in &self.slides {
            let slide_name = &slide.slide_name;

            partial_ord.push(quote! {
                (#name::#slide_name {}, #name::#slide_name {}) => Some(std::cmp::Ordering::Equal),
            });
        }

        quote! {
            impl std::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    match (self, other) {
                        #(#partial_eq)*
                        _ => false
                    }
                }
            }

            impl std::cmp::PartialOrd for #name {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    match (self, other) {
                        #(#partial_ord)*
                        _ => None
                    }
                }
            }
        }
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
            impl<'a> core::convert::TryFrom<&'a str> for #name {
                type Error = <Self as std::str::FromStr>::Err;

                fn try_from(s: &'a str) -> Result<Self, Self::Error> {
                    s.parse()
                }
            }

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
        let mut next_tokens = Vec::new();
        let mut prev_tokens = Vec::new();
        let mut slide_number = Vec::new();

        for slide in &self.slides {
            matches.push(slide.slidable_match());
        }

        for (i, slide) in self.slides.iter().enumerate() {
            let slide_name = &slide.slide_name;
            let next_slide = match i {
                x if x + 1 < self.slides.len() => Some(&self.slides[x + 1]),
                _ => None,
            };
            let prev_slide = match i {
                0 => None,
                _ => self.slides.get(i - 1),
            };

            let next = match next_slide {
                Some(next_slide) => {
                    let next_slide_name = &next_slide.slide_name;
                    quote! {
                        Self::#slide_name {} => Some(Self::#next_slide_name {}),
                    }
                }
                None => quote! {
                    Self::#slide_name {} => None,
                },
            };

            let prev = match prev_slide {
                Some(prev_slide) => {
                    let prev_slide_name = &prev_slide.slide_name;
                    quote! {
                        Self::#slide_name {} => Some(Self::#prev_slide_name {}),
                    }
                }
                None => quote! {
                    Self::#slide_name {} => None,
                },
            };

            next_tokens.push(next);
            prev_tokens.push(prev);

            let number = i + 1;

            slide_number.push(quote! {
                Self::#slide_name {} => #number,
            });
        }

        let number_of_slides = slide_number.len();

        quote! {
            impl dioxus_slides::Slidable for #name where Self: Clone {
                fn render<'a>(&self, cx: &'a dioxus::prelude::ScopeState) -> dioxus::prelude::Element<'a> {
                    match self {
                        #(#matches)*
                        _ => None
                    }
                }

                fn next(&self) -> Option<Self> {
                    match self {
                        #(#next_tokens)*
                        _ => None
                    }
                }

                fn prev(&self) -> Option<Self> {
                    match self {
                        #(#prev_tokens)*
                        _ => None
                    }
                }

                fn slide_number(&self) -> usize {
                    match self {
                        #(#slide_number)*
                        _ => 0
                    }
                }

                fn number_of_slides(&self) -> usize {
                    #number_of_slides
                }
            }
        }
    }

    fn error_name(&self) -> Ident {
        Ident::new(&(self.name.to_string() + "MatchError"), Span::call_site())
    }

    fn error_type(&self) -> TokenStream2 {
        let match_error_name = self.error_name();

        let mut type_defs = Vec::new();
        let mut error_variants = Vec::new();

        for slide in &self.slides {
            let slide_name = &slide.slide_name;
            let slide_error_ident = format_ident!("{}ParseError", slide_name);
            // let mut slide_error_variants = Vec::new();

            type_defs.push(quote! {
                #[allow(non_camel_case_types)]
                #[derive(Debug, PartialEq)]
                pub enum #slide_error_ident {}
            });

            error_variants.push(quote! {
                #slide_name(#slide_error_ident)
            });
        }

        quote! {
            #(#type_defs)*

            #[allow(non_camel_case_types)]
            #[derive(Debug, PartialEq)]
            pub enum #match_error_name {
                #(#error_variants),*
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
