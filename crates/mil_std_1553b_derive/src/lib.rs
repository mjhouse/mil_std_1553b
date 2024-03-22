use proc_macro::{Span, TokenStream};
use proc_macro_error::{abort, abort_call_site};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Ident, ItemFn};

mod word;
mod field;

use field::FieldArgs;
use word::FieldKind;

/// Derive the `Word` trait and associated From implementations
/// 
/// This trait requires that the `Default` trait is implemented
/// as well. To avoid this, implement the Word trait manually.
/// 
/// ### Example
/// 
/// ```rust
/// #[derive(Default, Word)]
/// struct MyWord {
///  
///     #[data]
///     buffer: [u8; 2],
///
///     #[parity]
///     parity_bit: u8,
///
///     // you can have any other
///     // fields you need
///     count: u8,
/// }
/// ```
#[proc_macro_derive(Word, attributes(data,parity))]
#[proc_macro_error::proc_macro_error]
pub fn word_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let item = match input.data {
        syn::Data::Struct(s) => s,
        _ => abort_call_site!("Word derive is only valid on a struct")
    };

    let fields = match item.fields {
        syn::Fields::Named(n) => n,
        _ => abort_call_site!("Word derive requires named fields")
    };

    let mut data_field: FieldKind = FieldKind::None;
    let mut parity_field: FieldKind = FieldKind::None;

    for named in fields.named {
        match word::parse_field(&named) {
            
            word::FieldKind::Data(v) if data_field.is_some() => 
                abort!(v, "Duplicate #[data] fields"),
            
            word::FieldKind::Parity(v) if parity_field.is_some() => 
                abort!(v, "Duplicate #[parity] fields"),

            k if k.is_data() => data_field = k,
            k if k.is_parity() => parity_field = k,
            _ => continue
        }
    }

    if data_field.is_none() {
        abort!(data_field.ident(),"Need a '#[data]' field with type '[u8;2]'");
    }

    if parity_field.is_none() {
        abort!(parity_field.ident(),"Need a '#[parity]' field with type 'u8'");
    }

    let self_type = input.ident;

    let data_ident = data_field.ident();


    let word_impl = word::impl_word(&data_field, &parity_field, &self_type);

    quote!(
        #word_impl

        impl From<&mil_std_1553b::DataWord> for #self_type {
            fn from(word: &mil_std_1553b::DataWord) -> Self {
                use mil_std_1553b::Word;
                #self_type::new()
                    .with_bytes(word.as_bytes())
                    .with_parity(word.parity())
            }
        }

        impl From<&#self_type> for  mil_std_1553b::DataWord {
            fn from(word: &#self_type) -> Self {
                use mil_std_1553b::Word;
                mil_std_1553b::DataWord::new()
                    .with_bytes(word.as_bytes())
                    .with_parity(word.parity())
            }
        }

        impl From<mil_std_1553b::DataWord> for #self_type {
            fn from(word: mil_std_1553b::DataWord) -> Self {
                Self::from(&word)
            }
        }

        impl From<#self_type> for  mil_std_1553b::DataWord {
            fn from(word: #self_type) -> Self {
                Self::from(&word)
            }
        }

        impl From<&#self_type> for  u16 {
            fn from(word: &#self_type) -> Self {
                u16::from_be_bytes(word.#data_ident)
            }
        }

        impl From<#self_type> for  u16 {
            fn from(word: #self_type) -> Self {
                u16::from_be_bytes(word.#data_ident)
            }
        }
    ).to_token_stream().into()
}

/// Implement a parser for a word field
/// 
/// ### Arguments
/// 
/// * name - A const name for the field (e.g. 'MY_FIELD')
/// * mask - A u16 bit mask for the field (e.g. '0b10000000000000000')
/// 
/// ### Example
/// 
/// ```rust
/// 
/// #[field(MY_FIELD, 0b1000000000000000)]
/// pub fn get_my_field_value(&self) -> u8 { 
///     Self::MY_FIELD.get(self)
/// }
/// 
/// ```
/// 
/// Will generate the following code:
/// 
/// ```rust
/// 
/// pub const MY_FIELD_MASK: u16 = 32768u16;
/// pub const MY_FIELD: Field = Field::from(Self::MY_FIELD_MASK);
/// pub fn get_my_field_value(&self) -> u8 {
///     Self::MY_FIELD.get(self)
/// }
/// 
/// ```
#[proc_macro_attribute]
#[proc_macro_error::proc_macro_error]
pub fn field(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = FieldArgs::new(attr);
    let item = parse_macro_input!(item as ItemFn);

    let name = args.name();
    let mask = args.mask();

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