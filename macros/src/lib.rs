extern crate proc_macro;

use std::fmt::Display;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, parse::Parse, Token, Visibility, Ident};

struct ItemFunction {
    fn_token: Token![fn],
    ident: Ident,
    vis: Visibility,
}

impl Display for ItemFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ItemFunction(vis: '{}', name: {})", match self.vis {
            Visibility::Public(_) => "pub ",
            _ => "",
        }, self.ident)
    }
}

impl Parse for ItemFunction {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![fn]) {
            Ok(ItemFunction {
                fn_token: input.parse()?,
                ident: input.parse()?,
                vis: input.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

#[proc_macro_attribute]
pub fn update(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("{:?}", attr);
    let itemFn = parse_macro_input!(item as ItemFunction);
    println!("{}", itemFn);

    quote! {
        fn update(dt: f32) -> Result<(), String> {
            Ok(())
        }
    }.into()
}
