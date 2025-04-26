use std::fmt::Debug;
use syn::Token;
use syn::parse::Parse;
use syn::Ident;

pub struct InputStruct {
    pub name: Ident,
    pub _1: Token![:],
    pub value: i32,
}

impl Debug for InputStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "InputStruct {{ name: {}, _1: :, value: {} }}",
            self.name, self.value
        )
    }
}

impl Parse for InputStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = input.parse()?;
        let _1: Token![:] = input.parse()?;
        let fields: syn::LitInt = input.parse()?;
        Ok(InputStruct {
            name,
            _1,
            value: fields.base10_parse().unwrap(),
        })
    }
}

impl ToString for InputStruct {
    fn to_string(&self) -> String {
        format!("{}: {}", self.name, self.value)
    }
}
impl InputStruct{
    pub fn parse_string(s: String) -> syn::Result<Self> {
        syn::parse_str(&s)
    }
}