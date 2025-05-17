#![cfg(test)]

use crate::tests::utils::Expected;

use super::super::derive_serialize_impl;

use pretty_assertions::assert_eq;
use quote::quote;

#[test]
fn variant_types_serialization() {
    let r#enum = quote! {
        enum VariantTypes {
            Unit,
            Newtype(u8),
            Unnamed(u8, u16),
            Field{ a: u8, b: u16 },
        }
    };
    let expected = crate::tests::utils::str_to_expected(include_str!("../../tests/objects/enum_variant_types.rs"), &Expected::Serialize);
    let result = derive_serialize_impl(r#enum);
    assert_eq!(result.to_string(), expected);
}

#[test]
fn unnamed_field_serialization() {
    let r#enum = quote! {
        enum UnnamedField {
            Array([u16; 12]),
        }
    };
    let expected = crate::tests::utils::str_to_expected(include_str!("../../tests/objects/enum_unnamed_field.rs"), &Expected::Serialize);
    let result = derive_serialize_impl(r#enum);
    assert_eq!(result.to_string(), expected);
}

#[test]
fn named_field_serialization() {
    let r#enum = quote! {
        enum NamedField {
            Array { inner: [u16; 12] },
        }
    };
    let expected = crate::tests::utils::str_to_expected(include_str!("../../tests/objects/enum_named_field.rs"), &Expected::Serialize);
    let result = derive_serialize_impl(r#enum);
    assert_eq!(result.to_string(), expected);
}
