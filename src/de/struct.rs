//! Generates serialize for a struct.
mod tests;

use super::ItemType;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident};

/// The primary derive implementation.
///
/// * `item`: The item being derived.
pub fn derive(item: &DeriveInput) -> TokenStream {
    let fields: &Fields = if let Data::Struct(s) = &item.data {
        &s.fields
    } else {
        panic!("Only structs are supported");
    };

    let item_ident = &item.ident;
    let seq_body = generate_seq_body(item_ident, fields);
    let body = generate_body(item_ident, fields);
    quote! {
        impl<'de> serde::de::Deserialize<'de> for #item_ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct Visitor;
                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = #item_ident;

                    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                        formatter.write_str(stringify!(struct #item_ident))
                    }

                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::SeqAccess<'de>,
                    {
                        #seq_body
                    }
                }

                #body
            }
        }
    }
}

/// Generates the body for the deserialize method.
///
/// * `item_ident`: The name of the item being generated.
/// * `fields`: The fields of the struct.
fn generate_body(item_ident: &Ident, fields: &Fields) -> TokenStream {
    match fields {
        Fields::Unnamed(..) | Fields::Named(..) => quote! {
            deserializer.deserialize_tuple_struct(stringify!(#item_ident), usize::MAX, Visitor)
        },
        Fields::Unit => {
            quote! {
                deserializer.deserialize_unit_struct(stringify!(#item_ident), Visitor)
            }
        }
    }
}

/// Generates the body of of `serialize` for an struct.
///
/// * `ident`: The name of the struct.
/// * `fields`: The fields of the struct.
fn generate_seq_body(ident: &Ident, fields: &Fields) -> TokenStream {
    match &fields {
        syn::Fields::Named(fields_named) => super::generate_named_fields_deserialization(ident, ItemType::Struct, fields_named),
        syn::Fields::Unnamed(fields_unnamed) => super::generate_unnamed_fields_deserialization(ident, ItemType::Struct, fields_unnamed),
        syn::Fields::Unit => quote! {Ok(#ident)},
    }
}
