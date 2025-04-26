use proc_macro::TokenStream as Tok1;
use quote::quote;
use syn::parse;

mod structs;

#[proc_macro]
pub fn to_hash_map(input: Tok1) -> Tok1 {
    let input: structs::InputStruct = parse(input).unwrap();
    eprintln!("{input:?}");
    let name = input.name.to_string();
    let value = input.value;
    quote!(
        {
            let mut map = ::std::collections::HashMap::new();
            map.insert(#name, #value);
            map
        }
    )
    .into()
}
