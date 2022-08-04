use near_abi_client::__private::{generate_abi_client, read_abi};
use quote::{format_ident, quote};
use std::path::PathBuf;

fn camel_to_snake(s: &str) -> String {
    let mut words = vec![];
    let mut buf = String::new();
    let mut last_upper = false;
    for c in s.chars() {
        if !buf.is_empty() && !last_upper && c.is_uppercase() {
            words.push(buf);
            buf = String::new();
        }
        last_upper = c.is_uppercase();
        buf.push(c.to_ascii_lowercase());
    }
    words.push(buf);
    words.join("_")
}

#[proc_macro]
pub fn near_abi_client(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let abi_def = syn::parse_macro_input!(tokens as AbiDef);
    let near_abi = read_abi(PathBuf::from(&abi_def.path.value()));

    let client = generate_abi_client(near_abi, abi_def.name.clone());

    let contract_name = format_ident!("{}", abi_def.name);
    let mod_name = format_ident!("__{}", camel_to_snake(&abi_def.name.to_string()));

    quote! {
        pub mod #mod_name {
            #client
        }
        use #mod_name::#contract_name;
    }
    .into()
}

struct AbiDef {
    /// Resulting client struct name.
    name: syn::Ident,
    /// Path to the ABI file.
    path: syn::LitStr,
}

impl syn::parse::Parse for AbiDef {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        input.parse::<syn::Token![type]>()?;
        let name = input.parse::<syn::Ident>()?;
        input.parse::<syn::Token![for]>()?;
        let path = input.parse()?;
        Ok(AbiDef { name, path })
    }
}
