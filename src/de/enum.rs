//! Generates serialize for a enum.
mod tests;

use super::ItemType;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident, Variant};

/// The primary derive implementation.
///
/// * `item`: The item being derived.
pub fn derive(item: &DeriveInput) -> TokenStream {
    let variants: Vec<Variant> = if let Data::Enum(s) = &item.data {
        s.variants.clone().into_iter().collect()
    } else {
        panic!("Only enums are supported");
    };

    let item_ident = &item.ident;
    let seq_body = generate_seq_body(item_ident, &variants);
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
                        formatter.write_str(stringify!(enum #item_ident))
                    }

                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::SeqAccess<'de>,
                    {
                        #seq_body
                    }
                }

                deserializer.deserialize_tuple_struct(stringify!(#item_ident), usize::MAX, Visitor)
            }
        }
    }
}

/// Generates the body of of `deserialize` for an enum.
///
/// * `ident`: The name of the item.
/// * `variants`: The variants of the enum.
fn generate_seq_body(item_ident: &Ident, variants: &[Variant]) -> TokenStream {
    let fieldless_enum_ident = format_ident!("{item_ident}_");
    let arms: Vec<_> = variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            let de = match &variant.fields {
                Fields::Named(fields_named) => super::generate_named_fields_deserialization(&quote! {#item_ident::#ident}, ItemType::Enum, fields_named),
                Fields::Unnamed(fields_unnamed) => super::generate_unnamed_fields_deserialization(&quote! {#item_ident::#ident}, ItemType::Enum, fields_unnamed),
                Fields::Unit => {
                    return quote! { #fieldless_enum_ident::#ident => Ok(#item_ident::#ident), };
                }
            };
            quote! {
                #fieldless_enum_ident::#ident => {
                    #de
                }
            }
        })
        .collect();

    let error_string = format!("enum {item_ident} discriminant");
    let variant_names: Vec<_> = variants.iter().map(|v| v.ident.to_string()).collect();
    quote! {
        type DiscriminantType = <#item_ident as discrimin_ant::Discriminantable>::Discriminant;
        let discriminant = serde::de::SeqAccess::next_element::<DiscriminantType>(&mut seq)?.ok_or_else(|| serde::de::Error::invalid_length(0usize, &#error_string))?;
        let variant = #fieldless_enum_ident::try_from(discriminant).map_err(|()| serde::de::Error::unknown_variant(&discriminant.to_string(), &[#(#variant_names),*]))?;
        match variant {
            #(#arms)*
        }
    }
}
