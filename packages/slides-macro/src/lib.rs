use proc_macro::TokenStream;
use quote::quote;
use syn::{
    __private::TokenStream2,
    {parse_macro_input, Ident},
};

extern crate proc_macro;

#[proc_macro_derive(Slidable)]
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
}

impl SlideEnum {
    fn parse(data: syn::ItemEnum) -> syn::Result<Self> {
        let name = &data.ident;

        let myself = Self { name: name.clone() };

        Ok(myself)
    }

    fn impl_display(&self) -> TokenStream2 {
        let name = &self.name;

        quote! {
            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    Ok(())
                }
            }
        }
    }

    fn parse_impl(&self) -> TokenStream2 {
        let name = &self.name;

        quote! {
            impl std::str::FromStr for #name {
                type Err = String;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Ok(Self {})
                }
            }
        }
    }

    fn slidable_impl(&self) -> TokenStream2 {
        let name = &self.name;

        quote! {
            impl Slidable for #name {
                fn render<'a>(&self, cx: &'a ScopeState) -> Element<'a> {
                    render! {
                        div {
                            h1 { "Hello, world!" }
                            p { "This is a slide." }
                        }
                    }
                }
            }
        }
    }
}
