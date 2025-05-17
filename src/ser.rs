//! Provides serialization methods.
mod r#enum;
mod r#struct;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Data, DeriveInput, Field, Ident};

/// The actual implementation for `derive_serialize`.
pub fn derive_serialize_impl(item: TokenStream) -> TokenStream {
    #[expect(clippy::expect_used)]
    let item: DeriveInput = syn::parse2(item).expect("to be able to parse tokens");
    match &item.data {
        Data::Enum(..) => r#enum::derive(&item),
        Data::Struct(..) => r#struct::derive(&item),
        Data::Union(..) => panic!("Unions are not supported"),
    }
}

/// Generates the serialization for a field.
///
/// * `serializer_ident`: The `Ident` for the serializer.
/// * `field_accessor`: How the field is accessed.
/// * `field`: The `Field` to use.
fn generate_field_serialization(serializer_ident: &Ident, field_accessor: &impl ToTokens, field: &Field) -> TokenStream {
    if let syn::Type::Array(..) = &field.ty {
        quote! { #field_accessor.iter().try_for_each(|elem| s.serialize_field(stringify!(#field_accessor), elem))?; }
    } else {
        quote! { #serializer_ident.serialize_field(stringify!(#field_accessor), &#field_accessor)?; }
    }
}
