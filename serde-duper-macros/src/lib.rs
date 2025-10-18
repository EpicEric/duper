use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Item, ItemStruct, Meta, parse_macro_input};

#[proc_macro]
pub fn duper(input: TokenStream) -> TokenStream {
    // We expect an Item (struct) inside the macro invocation
    let item = parse_macro_input!(input as Item);

    match item {
        Item::Struct(s) => expand_struct(s).into(),
        other => syn::Error::new_spanned(other, "duper! only supports struct items")
            .to_compile_error()
            .into(),
    }
}

/// Expand the struct: preserve it, but for fields with #[duper(Name)] generate modules and #[serde(with = "...")]
fn expand_struct(mut s: ItemStruct) -> proc_macro2::TokenStream {
    let struct_ident = s.ident.clone();
    let mut modules = Vec::<proc_macro2::TokenStream>::new();

    let (has_serialize, has_deserialize) = has_serde_derive_attributes(&s.attrs);

    if let syn::Fields::Named(ref mut fields_named) = s.fields {
        for field in fields_named.named.iter_mut() {
            // Remove the #[duper(...)] attribute and capture it
            let mut maybe_duper: Option<Attribute> = None;
            field.attrs.retain(|attr| {
                if attr.path().is_ident("duper") {
                    maybe_duper = Some(attr.clone());
                    false
                } else {
                    true
                }
            });

            if let Some(duper_attr) = maybe_duper {
                let duper_name = match parse_duper_name(&duper_attr) {
                    Some(n) => n,
                    None => {
                        let err = syn::Error::new_spanned(
                            duper_attr,
                            "expected #[duper(Name<...>)] or #[duper(Name)]",
                        );
                        modules.push(err.to_compile_error());
                        continue;
                    }
                };

                let field_ident = field.ident.as_ref().unwrap();
                let mod_ident = format_ident!("__serde_duper_{}_{}", struct_ident, field_ident);
                let path_string = format!("{}", mod_ident);

                let serde_with_attr: Attribute = syn::parse_quote! {
                    #[serde(with = #path_string)]
                };
                field.attrs.push(serde_with_attr);

                let ty = &field.ty;
                let module =
                    generate_module(&mod_ident, ty, &duper_name, has_serialize, has_deserialize);
                modules.push(module);
            }
        }
    }

    quote! {
        #s

        #(#modules)*
    }
}

fn has_serde_derive_attributes(attrs: &[Attribute]) -> (bool, bool) {
    let mut has_serialize = false;
    let mut has_deserialize = false;
    for attr in attrs {
        if attr.path().is_ident("derive") {
            if let Meta::List(list) = &attr.meta {
                let _ = list.parse_nested_meta(|nested| {
                    if let Some(ident) = nested.path.get_ident().map(ToString::to_string) {
                        if ident.ends_with("Serialize") {
                            has_serialize = true;
                        } else if ident.ends_with("Deserialize") {
                            has_deserialize = true;
                        }
                    }
                    Ok(())
                });
            }
        }
    }
    (has_serialize, has_deserialize)
}

/// Parse an attribute like `#[duper(CustomMap<_, (_, CoolVec<_>)>)]`
/// and return the top identifier name "CustomMap".
fn parse_duper_name(attr: &Attribute) -> Option<String> {
    if let Meta::List(list) = &attr.meta {
        let mut result: Option<String> = None;
        let _ = list.parse_nested_meta(|nested| {
            if let Some(ident) = nested.path.get_ident() {
                result = Some(ident.to_string());
            }
            Ok(())
        });
        result
    } else {
        None
    }
}

fn generate_module(
    mod_ident: &proc_macro2::Ident,
    ty: &syn::Type,
    duper_name: &str,
    has_serialize: bool,
    has_deserialize: bool,
) -> proc_macro2::TokenStream {
    let mut module_tokens = Vec::new();

    if has_serialize {
        let serialize_fn = quote! {
            pub fn serialize<S>(value: &#ty, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_newtype_struct(#duper_name, &value)
            }
        };
        module_tokens.push(serialize_fn);
    }

    if has_deserialize {
        let deserialize_fn = quote! {
            pub fn deserialize<'de, D>(deserializer: D) -> Result<#ty, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = #ty;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("a newtype struct")
                    }

                    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        Ok(serde::Deserialize::deserialize(deserializer)?)
                    }
                }

                deserializer.deserialize_newtype_struct(#duper_name, Visitor)
            }
        };
        module_tokens.push(deserialize_fn);
    }

    if !module_tokens.is_empty() {
        quote! {
            #[allow(non_snake_case)]
            mod #mod_ident {
                use super::*;

                #(#module_tokens)*
            }
        }
    } else {
        // Generate empty module
        quote! {
            #[allow(non_snake_case)]
            mod #mod_ident {}
        }
    }
}
