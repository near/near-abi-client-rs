use near_abi_client::__private::{generate_abi_client, read_abi};
use std::path::PathBuf;

#[proc_macro]
pub fn near_abi_client(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
        input.parse::<syn::Token![type]>()?;
        let name = input.parse::<syn::Ident>()?;
        input.parse::<syn::Token![for]>()?;
        let path = input.parse()?;
        Ok(AbiDef { name, path })
    }
}
