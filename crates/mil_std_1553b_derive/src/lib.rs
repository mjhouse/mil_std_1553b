use proc_macro::{Span, TokenStream, TokenTree};
use proc_macro_error::{abort_call_site, OptionExt};
use quote::{quote, ToTokens};
use std::collections::HashSet as Set;
use std::fmt::format;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Ident, ItemFn, LitInt, LitStr, MetaNameValue, Token};
use std::str::FromStr;

// fn impl_word(ast: &syn::DeriveInput) -> TokenStream {
//     let name = &ast.ident;
//     let gen = quote! {
//         impl HelloMacro for #name {
//             fn hello_macro() {
//                 println!("Hello, Macro! My name is {}!", stringify!(#name));
//             }
//         }
//     };
//     gen.into()
// }

#[proc_macro_derive(Word, attributes(data,parity))]
pub fn word_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()

    // // Construct a representation of Rust code as a syntax tree
    // // that we can manipulate
    // let ast = syn::parse(input).unwrap();

    // // Build the trait implementation
    // impl_word(&ast)
}

struct FieldArgs {
    name: Option<String>,
    value: Option<String>
}

impl FieldArgs {
    fn new(input: TokenStream) -> Self {
        let mut items = input.into_iter();

        let name = items
            .next()
            .map(|t| t.to_string());

        // skip the comma
        items.next();

        let value = items
            .next()
            .map(|t| t
                .to_string()
                .trim_start_matches("0b")
                .to_string());
        
        Self { name, value }
    }

    fn name(&self) -> String {
        self.name
            .clone()
            .expect_or_abort("Need an identifier for the field")
    }

    fn mask(&self) -> u16 {
        self.value
            .clone()
            .and_then(|v| u16::from_str_radix(&v,2).ok())
            .expect_or_abort("Need a mask for the field")
    }

}

#[proc_macro_attribute]
#[proc_macro_error::proc_macro_error]
pub fn word_field(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = FieldArgs::new(attr);
    let item = parse_macro_input!(item as ItemFn);

    let name = args.name();
    let mask = args.mask();

    // println!("NAME: {}",name);
    // println!("MASK: {:016b}",mask);

    let mask_label = format!("{}_MASK",name);
    let mask_ident = Ident::new(&mask_label, Span::call_site().into());

    let field_label = format!("{}",name);
    let field_ident = Ident::new(&field_label, Span::call_site().into());

    quote!(
        pub const #mask_ident: u16 = #mask;

        pub const #field_ident: Field = Field::from(Self::#mask_ident);

        #item
    ).to_token_stream().into()
}