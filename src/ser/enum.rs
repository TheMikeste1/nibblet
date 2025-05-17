//! Generates serialize for a enum.
mod tests;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{Data, DeriveInput, FieldsNamed, FieldsUnnamed, Ident, Variant};

/// The primary derive implementation.
///
/// * `item`: The item being derived.
pub fn derive(item: &DeriveInput) -> TokenStream {
    let variants: Vec<Variant> = if let Data::Enum(e) = &item.data {
        e.variants.clone().into_iter().collect()
    } else {
        panic!("Only enums are supported");
    };

    let item_ident = &item.ident;
    let item_span = item_ident.span();

    let assert_discriminantable = quote_spanned! {item_span=>
        struct _AssertDiscriminantable where #item_ident: discrimin_ant::Discriminantable;
    };

    let body = generate_serialize_body(item_ident, &variants);
    quote! {
        #assert_discriminantable

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
/// * `variants`: The variants of the enum.
fn generate_serialize_body(ident: &Ident, variants: &[Variant]) -> TokenStream {
    let num_fields_calculation = generate_num_fields_calculation(variants);

    let serializer_ident = format_ident!("s");
    let serialize_variant_arms = variants.iter().map(|v| {
        let ident = &v.ident;
        match &v.fields {
            syn::Fields::Named(fields_named) => generate_named_match_arm(&serializer_ident, ident, fields_named),
            syn::Fields::Unnamed(fields_unnamed) => generate_unnamed_match_arm(&serializer_ident, ident, fields_unnamed),
            syn::Fields::Unit => quote! { Self::#ident => {} },
        }
    });

    quote! {
        let num_fields: usize = #num_fields_calculation;

        use serde::ser::SerializeStruct;
        let mut #serializer_ident = serializer.serialize_struct(stringify!(#ident), num_fields)?;
        let discriminant = discrimin_ant::Discriminantable::discriminant(self);
        #serializer_ident.serialize_field("__discriminant__", &discriminant)?;
        match self {
            #( #serialize_variant_arms )*
        }

        #serializer_ident.end()
    }
}

/// Generates the match arm for an enum variant with unnamed fields, e.g. `Unnamed(u8, u16)`.
///
/// * `serializer_ident`: The name of the serializer.
/// * `variant_ident`: The name of the variant.
/// * `fields`: The fields of the variant.
fn generate_unnamed_match_arm(serializer_ident: &Ident, variant_ident: &Ident, fields: &FieldsUnnamed) -> TokenStream {
    let (field_idents, serializations): (Vec<_>, Vec<_>) = fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let ident = format_ident!("field_{i}");
            let serialization = super::generate_field_serialization(serializer_ident, &ident, field);
            (ident, serialization)
        })
        .unzip();
    quote! {
        Self::#variant_ident ( #( #field_idents ),* ) => {
            #( #serializations )*
        }
    }
}

/// Generates the match arm for an enum variant with named fields, e.g. `Named{ a: u8, b: u16 }`.
///
/// * `serializer_ident`: The name of the serializer.
/// * `variant_ident`: The name of the variant.
/// * `fields`: The fields of the variant.
fn generate_named_match_arm(serializer_ident: &Ident, variant_ident: &Ident, fields: &FieldsNamed) -> TokenStream {
    let (field_idents, serializations): (Vec<_>, Vec<_>) = fields
        .named
        .iter()
        .map(|field| {
            #[expect(clippy::expect_used)]
            let ident = field.ident.clone().expect("for a named field to have an ident");
            let serialization = super::generate_field_serialization(serializer_ident, &ident, field);
            (ident, serialization)
        })
        .unzip();

    quote! {
        Self::#variant_ident { #( #field_idents ),* } => {
            #( #serializations )*
        }
    }
}

/// Generates code to calculate the number of fields needed to serialize an enum, including the
/// discriminant.
///
/// * `variants`: The variants in the enum.
fn generate_num_fields_calculation(variants: &[Variant]) -> TokenStream {
    let match_arms = variants
        .iter()
        .map(|v| {
            let ident = &v.ident;
            match &v.fields {
                syn::Fields::Named(fields) => {
                    let num_fields: usize = fields.named.len();
                    quote! {
                        Self::#ident{ .. } => #num_fields
                    }
                }
                syn::Fields::Unnamed(fields) => {
                    let num_fields: usize = fields.unnamed.len();
                    quote! {
                        Self::#ident(..) => #num_fields
                    }
                }
                syn::Fields::Unit => quote! {
                    Self::#ident => 0usize
                },
            }
        })
        .collect::<Vec<_>>();

    quote! {
        match self {
            #( #match_arms, )*
        }
        .checked_add(1usize)
        .unwrap_or_else(|| panic!("for there to be less than {} fields in an enum variant", usize::MAX))
    }
}
