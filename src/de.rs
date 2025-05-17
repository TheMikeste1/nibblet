//! Provides deserialization methods.
mod r#enum;
mod r#struct;

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Data, DeriveInput, Field, FieldsNamed, FieldsUnnamed};

/// Specifies which type of object is being generated.
#[derive(Copy, Clone)]
enum ItemType {
    /// A struct is being generated.
    Struct,
    /// An enum is being generated.
    Enum,
}

/// The actual implementation for `derive_deserialize`.
pub fn derive_deserialize_impl(item: TokenStream) -> TokenStream {
    #[expect(clippy::expect_used)]
    let item: DeriveInput = syn::parse2(item).expect("to be able to parse tokens");
    match &item.data {
        Data::Enum(..) => r#enum::derive(&item),
        Data::Struct(..) => r#struct::derive(&item),
        Data::Union(..) => panic!("Unions are not supported"),
    }
}

/// Generates the deserialization for a field.
///
/// * `item_ident`: The name of the item being generated.
/// * `item_type`: The type of the item being generated.
/// * `field_ident`: The named of the field.
/// * `field`: The field information.
/// * `field_index`: The index of the field.
/// * `total_fields`: The total number of fields.
fn generate_field_deserialization(item_ident: &dyn ToTokens, item_type: ItemType, field_ident: &impl ToTokens, field: &Field, field_index: usize, total_fields: usize) -> TokenStream {
    let ident_string = quote! {#item_ident}.to_string().replace(" :: ", "::");
    let error_string = format!(
        "{} {ident_string} with {total_fields} element{}",
        {
            match item_type {
                ItemType::Struct => "struct",
                ItemType::Enum => "enum",
            }
        },
        if total_fields == 1 { "" } else { "s" }
    );
    if let syn::Type::Array(array_data) = &field.ty {
        let len = &array_data.len;
        quote! {
            let mut #field_ident = [0; #len];
            for elem in #field_ident.as_mut() {
                *elem = serde::de::SeqAccess::next_element::<_>(&mut seq)?
                    .ok_or_else(|| serde::de::Error::invalid_length(#field_index, &#error_string))?;
            }
        }
    } else {
        quote! {
            let #field_ident = serde::de::SeqAccess::next_element::<_>(&mut seq)?
                .ok_or_else(|| serde::de::Error::invalid_length(#field_index, &#error_string))?;
        }
    }
}

/// Generate code to serialize named fields.
///
/// * `item_ident`: The name of the item being generated.
/// * `item_type`: The type of the item.
/// * `fields`: The fields of the variant.
fn generate_named_fields_deserialization(item_ident: &dyn ToTokens, item_type: ItemType, fields: &FieldsNamed) -> TokenStream {
    let field_de = fields
        .named
        .iter()
        .enumerate()
        .map(|(i, field)| {
            #[expect(clippy::expect_used)]
            let ident = field.ident.clone().expect("for a named field to have an ident");
            generate_field_deserialization(item_ident, item_type, &ident, field, i, fields.named.len())
        })
        .reduce(|acc, field_de| {
            quote! {
                #acc
                #field_de
            }
        });

    let field_idents = fields.named.iter().map(|field| unsafe { field.ident.clone().unwrap_unchecked() });
    quote! {
        #field_de
        Ok(#item_ident { #(#field_idents),* })
    }
}

/// Generate code to serialize unnamed fields.
///
/// * `item_ident`: The name of the item being generated.
/// * `item_type`: The type of the item.
/// * `fields`: The fields of the variant.
fn generate_unnamed_fields_deserialization(item_ident: &dyn ToTokens, item_type: ItemType, fields: &FieldsUnnamed) -> TokenStream {
    let (field_idents, deserializations): (Vec<_>, Vec<_>) = fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let ident = format_ident!("field_{i}");
            let de = generate_field_deserialization(item_ident, item_type, &ident, field, i, fields.unnamed.len());
            (ident, de)
        })
        .unzip();
    let field_de = deserializations.into_iter().reduce(|acc, field_de| {
        quote! {
            #acc
            #field_de
        }
    });

    quote! {
        #field_de
        Ok(#item_ident ( #(#field_idents),* ))
    }
}
