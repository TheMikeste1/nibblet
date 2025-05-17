//! Generates serialize for a struct.
mod tests;

use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident};

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
    let body = generate_serialize_body(item_ident, fields);
    quote! {
        impl serde::ser::Serialize for #item_ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                #body
            }
        }
    }
}

/// Generates the body of of `serialize` for an enum.
///
/// * `ident`: The name of the item.
/// * `fields`: The variants of the enum.
fn generate_serialize_body(ident: &Ident, fields: &Fields) -> TokenStream {
    let num_fields = fields.len();

    let serializer_ident = format_ident!("s");
    let serializations = match &fields {
        syn::Fields::Named(fields_named) => generate_named_fields_serializations(&serializer_ident, fields_named),
        syn::Fields::Unnamed(fields_unnamed) => generate_unnamed_fields_serializations(&serializer_ident, fields_unnamed),
        syn::Fields::Unit => vec![],
    };

    quote! {
        use serde::ser::SerializeStruct;
        let mut #serializer_ident = serializer.serialize_struct(stringify!(#ident), #num_fields)?;
        #( #serializations )*
        #serializer_ident.end()
    }
}

/// Generate code to serialize named fields.
///
/// * `serializer_ident`: The name of the serializer.
/// * `fields`: The fields of the variant.
fn generate_named_fields_serializations(serializer_ident: &Ident, fields: &FieldsNamed) -> Vec<TokenStream> {
    fields
        .named
        .iter()
        .map(|field| {
            #[expect(clippy::expect_used)]
            let ident = field.ident.clone().expect("for a named field to have an ident");
            let ident = quote! {self.#ident};
            super::generate_field_serialization(serializer_ident, &ident, field)
        })
        .collect()
}

/// Generate code to serialize unnamed fields.
///
/// * `serializer_ident`: The name of the serializer.
/// * `fields`: The fields of the struct.
fn generate_unnamed_fields_serializations(serializer_ident: &Ident, fields: &FieldsUnnamed) -> Vec<TokenStream> {
    fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let i = Literal::usize_unsuffixed(i);
            let ident = quote! { self.#i };
            super::generate_field_serialization(serializer_ident, &ident, field)
        })
        .collect()
}
