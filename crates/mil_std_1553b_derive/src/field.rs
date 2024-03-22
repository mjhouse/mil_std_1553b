use proc_macro::TokenStream;
use proc_macro_error::OptionExt;

pub struct FieldArgs {
    name: Option<String>,
    value: Option<String>
}

impl FieldArgs {
    pub fn new(input: TokenStream) -> Self {
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

    pub fn name(&self) -> String {
        self.name
            .clone()
            .expect_or_abort("Need an identifier for the field")
    }

    pub fn mask(&self) -> u16 {
        self.value
            .clone()
            .and_then(|v| u16::from_str_radix(&v,2).ok())
            .expect_or_abort("Need a mask for the field")
    }

}