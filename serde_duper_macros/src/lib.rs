#![doc(
    html_logo_url = "https://raw.githubusercontent.com/EpicEric/duper/refs/heads/main/logos/duper-100-100.png"
)]
//! Macros for [`serde_duper`](https://docs.rs/serde_duper/).

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Fields, Ident, Item, ItemStruct, Meta, parse_macro_input};

#[proc_macro]
/// A proc-macro that automatically generates remote serializers and
/// deserializers for struct fields annotated with `#[duper(...)]`.
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use serde_duper_macros::duper;
///
/// duper! {
///     #[derive(Serialize, Deserialize)]
///     struct User {
///         #[duper(MyId)]
///         id: u64,
///         #[duper(AliasList)]
///         aliases: Vec<String>,
///     }
/// }
///
/// let u = User {
///     id: 1234,
///     aliases: vec!["duper".to_string()],
/// };
/// ```
///
/// Upon serializing and deserializing, `id` and `aliases` will be treated as
/// newtype structs. This is useful for adding identifiers to Duper values.
///
pub fn duper(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    match item {
        Item::Struct(s) => expand_struct(s).into(),
        other => syn::Error::new_spanned(other, "duper! only supports struct items")
            .to_compile_error()
            .into(),
    }
}

fn expand_struct(mut s: ItemStruct) -> proc_macro2::TokenStream {
    let struct_ident = s.ident.clone();
    let mut modules = Vec::<proc_macro2::TokenStream>::new();

    let (has_serialize, has_deserialize) = has_serde_derive_attributes(&s.attrs);

    if !&s.generics.params.is_empty() {
        return syn::Error::new_spanned(s.generics, "duper! doesn't support generic parameters")
            .into_compile_error();
    }

    match &mut s.fields {
        Fields::Named(fields_named) => {
            for field in fields_named.named.iter_mut() {
                process_field(
                    field,
                    format_ident!(
                        "__serde_duper_{}_{}",
                        struct_ident,
                        field.ident.as_ref().unwrap()
                    ),
                    &mut modules,
                    has_serialize,
                    has_deserialize,
                );
            }
        }
        Fields::Unnamed(fields_named) => {
            for (i, field) in fields_named.unnamed.iter_mut().enumerate() {
                process_field(
                    field,
                    format_ident!("__serde_duper_{}_{}", struct_ident, i),
                    &mut modules,
                    has_serialize,
                    has_deserialize,
                );
            }
        }
        Fields::Unit => (),
    }

    quote! {
        #s

        #(#modules)*
    }
}

fn process_field(
    field: &mut syn::Field,
    mod_ident: Ident,
    modules: &mut Vec<proc_macro2::TokenStream>,
    has_serialize: bool,
    has_deserialize: bool,
) {
    // Remove the #[duper(...)] attribute and capture it
    let maybe_duper_i = field
        .attrs
        .iter()
        .enumerate()
        .find(|(_, attr)| attr.path().is_ident("duper"))
        .map(|(i, _)| i);

    if let Some(duper_attr) = maybe_duper_i.map(|i| field.attrs.swap_remove(i)) {
        let duper_name = match parse_duper_name(&duper_attr) {
            Some(n) => n,
            None => {
                let err = syn::Error::new_spanned(
                    duper_attr,
                    "expected #[duper(Name<...>)] or #[duper(Name)]",
                );
                modules.push(err.to_compile_error());
                return;
            }
        };

        let path_string = format!("{mod_ident}");

        let serde_with_attr: Attribute = syn::parse_quote! {
            #[serde(with = #path_string)]
        };
        field.attrs.push(serde_with_attr);

        let ty = &field.ty;
        let module = generate_module(&mod_ident, ty, &duper_name, has_serialize, has_deserialize);
        modules.push(module);
    }
}

fn has_serde_derive_attributes(attrs: &[Attribute]) -> (bool, bool) {
    let mut has_serialize = false;
    let mut has_deserialize = false;
    for attr in attrs {
        if attr.path().is_ident("derive")
            && let Meta::List(list) = &attr.meta
        {
            let _ = list.parse_nested_meta(|nested| {
                if let Some(segment) = nested.path.segments.last() {
                    let ident = &segment.ident;
                    if ident == "Serialize" {
                        has_serialize = true;
                    } else if ident == "Deserialize" {
                        has_deserialize = true;
                    }
                }
                Ok(())
            });
        }
    }
    (has_serialize, has_deserialize)
}

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
                where S: ::serde::Serializer,
            {
                serializer.serialize_newtype_struct(#duper_name, &value)
            }
        };
        module_tokens.push(serialize_fn);
    }

    if has_deserialize {
        let deserialize_fn = quote! {
            pub fn deserialize<'de, D>(deserializer: D) -> Result<#ty, D::Error>
                where D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor
                {
                    type Value = #ty;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        formatter.write_str("a newtype struct ")?;
                        formatter.write_str(#duper_name)?;
                        Ok(())
                    }

                    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                    where
                        D: ::serde::Deserializer<'de>,
                    {
                        ::serde::Deserialize::deserialize(deserializer)
                    }
                }

                deserializer.deserialize_newtype_struct(#duper_name, Visitor)
            }
        };
        module_tokens.push(deserialize_fn);
    }

    if module_tokens.is_empty() {
        // Generate empty module
        quote! {
            #[allow(non_snake_case)]
            mod #mod_ident {}
        }
    } else {
        quote! {
            #[allow(non_snake_case)]
            mod #mod_ident {
                use super::*;

                #(#module_tokens)*
            }
        }
    }
}
