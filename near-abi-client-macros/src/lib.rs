use std::path::PathBuf;

#[path = "../../near-abi-client/src/abi_core.rs"]
mod abi_core;

use abi_core::{generate_abi_client, read_abi};

#[proc_macro]
pub fn generate(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let abi_def = syn::parse_macro_input!(tokens as AbiDef);
    let near_abi = read_abi(PathBuf::from(&abi_def.path.value()));

    generate_abi_client(near_abi, abi_def.name).into()
}

struct AbiDef {
    /// Resulting client struct name.
    name: syn::Ident,
    /// Path to the ABI file.
    path: syn::LitStr,
}

impl syn::parse::Parse for AbiDef {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        input.parse::<syn::Token![for]>()?;
        let path = input.parse()?;
        Ok(AbiDef { name, path })
    }
}
