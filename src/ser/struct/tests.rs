#![cfg(test)]

use crate::tests::utils::Expected;

use super::super::derive_serialize_impl;

use pretty_assertions::assert_eq;
use quote::quote;

#[test]
fn unnamed_field_serialization() {
    let r#enum = quote! {
        struct UnnamedField([u16; 12], u32);
    };
    let expected = crate::tests::utils::str_to_expected(include_str!("../../tests/objects/struct_unnamed_field.rs"), &Expected::Serialize);
    let result = derive_serialize_impl(r#enum);
    assert_eq!(result.to_string(), expected);
}

#[test]
fn named_field_serialization() {
    let r#enum = quote! {
        struct NamedField {
            array: [u16; 12],
            number: u32,
        }
    };
    let expected = crate::tests::utils::str_to_expected(include_str!("../../tests/objects/struct_named_field.rs"), &Expected::Serialize);
    let result = derive_serialize_impl(r#enum);
    assert_eq!(result.to_string(), expected);
}
