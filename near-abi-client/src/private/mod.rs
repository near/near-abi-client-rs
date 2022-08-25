use std::path::{Path, PathBuf};

use near_abi::{AbiRoot, AbiType};
use quote::{format_ident, quote};
use schemafy_lib::{Expander, Generator, Schema};

pub fn generate_abi_client(
    near_abi: AbiRoot,
    contract_name: proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    let schema_json = serde_json::to_string(&near_abi.body.root_schema).unwrap();

    let generator = Generator::builder().with_input_json(&schema_json).build();
    let (mut token_stream, schema) = generator.generate_with_schema();
    let mut expander = Expander::new(None, "", &schema);

    token_stream.extend(quote! {
        pub struct #contract_name {
            pub contract: workspaces::Contract,
        }
    });

    let mut methods_stream = proc_macro2::TokenStream::new();
    for function in near_abi.body.functions {
        let name = format_ident!("{}", function.name);
        let param_names = function
            .params
            .iter()
            .map(|arg_param| format_ident!("{}", arg_param.name))
            .collect::<Vec<_>>();
        let params = function
            .params
            .iter()
            .zip(&param_names)
            .map(|(arg_param, arg_name)| {
                let arg_type = match &arg_param.typ {
                    AbiType::Json { type_schema } => expand_subschema(&mut expander, type_schema),
                    AbiType::Borsh { type_schema: _ } => panic!("Borsh is currently unsupported"),
                };
                quote! { #arg_name: #arg_type }
            })
            .collect::<Vec<_>>();
        let return_type = function
            .result
            .map(|r_type| match r_type {
                AbiType::Json { type_schema } => expand_subschema(&mut expander, &type_schema),
                AbiType::Borsh { type_schema: _ } => panic!("Borsh is currently unsupported"),
            })
            .unwrap_or_else(|| format_ident!("{}", "()"));
        let name_str = name.to_string();
        let args = if param_names.is_empty() {
            // Special case for parameter-less functions because otherwise the type for
            // `[]` is not inferrable.
            quote! { () }
        } else {
            quote! { [#(#param_names),*] }
        };
        if function.is_view {
            methods_stream.extend(quote! {
                pub async fn #name(
                    &self,
                    worker: &workspaces::Worker<impl workspaces::Network>,
                    #(#params),*
                ) -> anyhow::Result<#return_type> {
                    let result = self.contract
                        .call(worker, #name_str)
                        .args_json(#args)?
                        .view()
                        .await?;
                    result.json::<#return_type>()
                }
            });
        } else {
            methods_stream.extend(quote! {
                pub async fn #name(
                    &self,
                    worker: &workspaces::Worker<impl workspaces::Network>,
                    gas: workspaces::types::Gas,
                    deposit: workspaces::types::Balance,
                    #(#params),*
                ) -> anyhow::Result<#return_type> {
                    let result = self.contract
                        .call(worker, #name_str)
                        .args_json(#args)?
                        .gas(gas)
                        .deposit(deposit)
                        .transact()
                        .await?;
                    result.json::<#return_type>()
                }
            });
        }
    }

    token_stream.extend(quote! {
        impl #contract_name {
            #methods_stream
        }
    });

    token_stream
}

pub fn read_abi(abi_path: impl AsRef<Path>) -> AbiRoot {
    let abi_path = if abi_path.as_ref().is_relative() {
        let crate_root = get_crate_root().unwrap();
        crate_root.join(&abi_path)
    } else {
        PathBuf::from(abi_path.as_ref())
    };

    let abi_json = std::fs::read_to_string(&abi_path)
        .unwrap_or_else(|err| panic!("Unable to read `{}`: {}", abi_path.to_string_lossy(), err));

    serde_json::from_str::<AbiRoot>(&abi_json).unwrap_or_else(|err| {
        panic!(
            "Cannot parse `{}` as ABI: {}",
            abi_path.to_string_lossy(),
            err
        )
    })
}

fn get_crate_root() -> std::io::Result<PathBuf> {
    if let Ok(path) = std::env::var("CARGO_MANIFEST_DIR") {
        return Ok(PathBuf::from(path));
    }

    let current_dir = std::env::current_dir()?;

    for p in current_dir.ancestors() {
        if std::fs::read_dir(p)?
            .into_iter()
            .filter_map(Result::ok)
            .any(|p| p.file_name().eq("Cargo.toml"))
        {
            return Ok(PathBuf::from(p));
        }
    }

    Ok(current_dir)
}

fn schemars_schema_to_schemafy(schema: &schemars::schema::Schema) -> Schema {
    let schema_json = serde_json::to_string(&schema).unwrap();
    serde_json::from_str(&schema_json).unwrap_or_else(|err| {
        panic!(
            "Could not convert schemars schema to schemafy model: {}",
            err
        )
    })
}

fn expand_subschema(
    expander: &mut Expander,
    schema: &schemars::schema::Schema,
) -> proc_macro2::Ident {
    let schemafy_schema = schemars_schema_to_schemafy(schema);
    format_ident!("{}", expander.expand_type_from_schema(&schemafy_schema).typ)
}
