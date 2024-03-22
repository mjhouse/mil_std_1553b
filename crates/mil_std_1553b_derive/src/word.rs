use syn::{Expr, ExprLit, Field, Ident, Lit};
use proc_macro2::TokenStream;

use proc_macro_error::{abort, OptionExt};
use quote::quote;

const DATA_LABEL: &str = "data";
const PARITY_LABEL: &str = "parity";

const DATA_KIND: &str = "[u8;2]";
const PARITY_KIND: &str = "u8";

#[derive(Eq,PartialEq,Debug)]
pub(crate) enum FieldLabel {
    None,
    Data(Ident),
    Parity(Ident)
}

#[derive(Eq,PartialEq,Debug)]
pub(crate) enum FieldKind {
    None,
    Data(Ident),
    Parity(Ident)
}

impl FieldLabel {
    
    pub(crate) fn is_some(&self) -> bool {
        !matches!(self,FieldLabel::None)
    }
    
    pub(crate) fn is_kind(&self, kind: &FieldKind) -> bool {
        match (self,kind) {
            (Self::None,FieldKind::None) => true,
            (Self::Data(_),FieldKind::Data(_)) => true,
            (Self::Parity(_),FieldKind::Parity(_)) => true,
            _ => false
        }
    }

}

impl FieldKind {

    pub(crate) fn is_none(&self) -> bool {
        matches!(self,FieldKind::None)
    }

    pub(crate) fn is_some(&self) -> bool {
        !matches!(self,FieldKind::None)
    }

    pub(crate) fn ident(&self) -> Option<Ident> {
        match self {
            Self::Data(v) => Some(v.clone()),
            Self::Parity(v) => Some(v.clone()),
            _ => None
        }
    }

    pub(crate) fn is_data(&self) -> bool {
        matches!(self,Self::Data(_))
    }

    pub(crate) fn is_parity(&self) -> bool {
        matches!(self,Self::Parity(_))
    }

}

pub(crate) fn parse_label(field: &Field) -> FieldLabel {
    let mut kind = FieldLabel::None;
    for attr in field.attrs.iter() {
        let ident = attr.path.get_ident();

        // get the attribute as a field type
        let attr = match ident.map(Ident::to_string).as_deref() {
            Some(DATA_LABEL) => FieldLabel::Data(ident.unwrap().clone()),
            Some(PARITY_LABEL) => FieldLabel::Parity(ident.unwrap().clone()),
            _ => FieldLabel::None,
        };

        if attr != FieldLabel::None {

            // found multiple labels on the same field
            if kind != FieldLabel::None {
                abort!(ident, "Multiple declarations of field");
            }

            kind = attr;
        }

    }
    kind
}

pub(crate) fn parse_kind(field: &Field) -> FieldKind {

    let ident = field
        .ident
        .clone()
        .expect_or_abort("Cannot derive for struct with unnamed field");

    let kind = match &field.ty {
        syn::Type::Path(v) => v
            .path
            .segments
            .iter()
            .next()
            .map(|t| t.ident.to_string())
            .unwrap_or("".into())
            .to_string(),
        syn::Type::Array(v) => {
            
            let t = match v.elem.as_ref() {
                syn::Type::Path(s) => s.path.segments
                    .clone()
                    .into_iter()
                    .next()
                    .map(|t| t.ident.to_string())
                    .unwrap_or("".into())
                    .to_string(),
                _ => String::new()
            };

            let l = match &v.len {
                Expr::Lit(ExprLit { lit: Lit::Int(i), .. }) => i
                    .base10_parse::<usize>()
                    .unwrap_or(0),
                _ => 0,
            };

            format!("[{};{}]",t,l)
        },
        _ => String::new()
    };

    match kind.as_str() {
        DATA_KIND => FieldKind::Data(ident),
        PARITY_KIND => FieldKind::Parity(ident),
        _ => FieldKind::None
    }
}

pub(crate) fn parse_field(field: &Field) -> FieldKind {
    let field_label = parse_label(field);

    if field_label.is_some() {
        let field_kind = parse_kind(field);

        if field_label.is_kind(&field_kind) {
            return field_kind;
        }
    }

    FieldKind::None
}

pub(crate) fn impl_new(data_ident: &Ident, parity_ident: &Ident) -> TokenStream {
    quote!(
        fn new() -> Self {
            Self {
                #data_ident: [0, 0],
                #parity_ident: 1,
                ..Default::default()
            }
        }
    )
}

pub(crate) fn impl_word(data_field: &FieldKind, parity_field: &FieldKind, self_type: &Ident) -> TokenStream {
    let data_ident = data_field.ident().expect_or_abort("No identifier for data field");
    let parity_ident = parity_field.ident().expect_or_abort("No identifier for parity field");

    let new_impl = impl_new(&data_ident, &parity_ident);

    quote!(
        impl mil_std_1553b::Word for #self_type {
            
            #new_impl
        
            fn with_value(mut self, data: u16) -> Self {
                self.set_value(data);
                self
            }
        
            fn with_bytes(mut self, data: [u8; 2]) -> Self {
                self.set_bytes(data);
                self
            }
        
            fn with_parity(mut self, parity: u8) -> Self {
                self.set_parity(parity);
                self
            }
        
            fn with_calculated_parity(mut self) -> Self {
                self.#parity_ident = self.calculate_parity();
                self
            }
        
            fn build(self) -> mil_std_1553b::Result<Self> {
                if self.check_parity() {
                    Ok(self)
                } else {
                    Err(mil_std_1553b::Error::InvalidWord)
                }
            }
        
            fn from_value(data: u16) -> Self {
                Self::new().with_value(data).with_calculated_parity()
            }
        
            fn from_bytes(data: [u8; 2]) -> Self {
                Self::new().with_bytes(data)
            }
        
            fn as_bytes(&self) -> [u8; 2] {
                self.#data_ident
            }
        
            fn as_value(&self) -> u16 {
                self.into()
            }
        
            fn set_value(&mut self, data: u16) {
                self.#data_ident = data.to_be_bytes();
                self.#parity_ident = self.calculate_parity();
            }
        
            fn set_bytes(&mut self, data: [u8; 2]) {
                self.#data_ident = data;
                self.#parity_ident = self.calculate_parity();
            }
        
            fn parity(&self) -> u8 {
                self.#parity_ident
            }
        
            fn set_parity(&mut self, parity: u8) {
                self.#parity_ident = parity;
            }
        
            fn calculate_parity(&self) -> u8 {
                match self.as_value().count_ones() % 2 {
                    0 => 1,
                    _ => 0,
                }
            }
        
            fn check_parity(&self) -> bool {
                self.parity() == self.calculate_parity()
            }
        }
    )
}