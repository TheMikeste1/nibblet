#![allow(clippy::unwrap_used)]
use super::*;

mod bincode_ {
    use super::*;

    mod ser {
        use pretty_assertions::assert_eq;

        fn serialize<T>(val: T) -> Vec<u8>
        where
            T: serde::Serialize,
        {
            let config = bincode::config::standard().with_big_endian().with_fixed_int_encoding();
            bincode::serde::encode_to_vec(val, config).unwrap()
        }

        #[test]
        fn enum_named_field() {
            use super::r#enum::NamedField;
            let val = NamedField::Array {
                inner: [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
            };
            let result = serialize(val);
            assert_eq!(
                result,
                [
                    // discriminant
                    0, 0, //
                    // inner
                    0, 12, 0, 11, 0, 10, 0, 9, 0, 8, 0, 7, 0, 6, 0, 5, 0, 4, 0, 3, 0, 2, 0, 1,
                ]
            );
        }

        #[test]
        fn enum_unnamed_field() {
            use super::r#enum::UnnamedField;
            let val = UnnamedField::Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
            let result = serialize(val);
            assert_eq!(
                result,
                [
                    // discriminant
                    0, //
                    // [u16; 12]
                    0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0, 11, 0, 12,
                ]
            );
        }

        #[test]
        fn enum_variant_types_unit() {
            use super::r#enum::VariantTypes;
            let val = VariantTypes::Unit;
            let result = serialize(val);
            assert_eq!(
                result,
                [
                    // discriminant
                    0,
                ]
            );
        }

        #[test]
        fn enum_variant_types_newtype() {
            use super::r#enum::VariantTypes;
            let val = VariantTypes::Newtype(42);
            let result = serialize(val);
            assert_eq!(
                result,
                [
                    // discriminant
                    1, //
                    // u8
                    42
                ]
            );
        }

        #[test]
        fn enum_variant_types_unnamed() {
            use super::r#enum::VariantTypes;
            let val = VariantTypes::Unnamed(42, 168);
            let result = serialize(val);
            assert_eq!(
                result,
                [
                    // discriminant
                    2, //
                    // u8
                    42, //
                    // u16
                    0, 168
                ]
            );
        }

        #[test]
        fn enum_variant_types_field() {
            use super::r#enum::VariantTypes;
            let val = VariantTypes::Field { a: 42, b: 168 };
            let result = serialize(val);
            assert_eq!(
                result,
                [
                    // discriminant
                    3, //
                    // u8
                    42, //
                    // u16
                    0, 168
                ]
            );
        }

        #[test]
        fn struct_named_field() {
            use super::r#struct::NamedField;
            let val = NamedField {
                array: [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
                number: 16,
            };
            let result = serialize(val);
            assert_eq!(
                result,
                [
                    // array
                    0, 12, 0, 11, 0, 10, 0, 9, 0, 8, 0, 7, 0, 6, 0, 5, 0, 4, 0, 3, 0, 2, 0, 1, //
                    // number
                    0, 0, 0, 16,
                ]
            );
        }

        #[test]
        fn struct_unnamed_field() {
            use super::r#struct::UnnamedField;
            let val = UnnamedField([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], 16);
            let result = serialize(val);
            assert_eq!(
                result,
                [
                    // [u16; 12]
                    0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0, 11, 0, 12, //
                    // u32
                    0, 0, 0, 16,
                ]
            );
        }
    }

    mod de {
        use assert_matches::assert_matches;
        use pretty_assertions::assert_eq;

        fn deserialize<T>(data: &[u8]) -> (T, usize)
        where
            T: serde::de::DeserializeOwned,
        {
            let config = bincode::config::standard().with_big_endian().with_fixed_int_encoding();
            bincode::serde::decode_from_slice(data, config).unwrap()
        }

        #[test]
        fn enum_named_field() {
            use super::r#enum::NamedField;
            let data = [
                // discriminant
                0, 0, //
                // inner
                0, 12, 0, 11, 0, 10, 0, 9, 0, 8, 0, 7, 0, 6, 0, 5, 0, 4, 0, 3, 0, 2, 0, 1,
            ];
            let (result, bytes_used) = deserialize::<NamedField>(&data);
            assert_eq!(bytes_used, data.len());
            assert_matches!(
                result,
                NamedField::Array { inner } if inner == [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
            );
        }

        #[test]
        fn enum_unnamed_field() {
            use super::r#enum::UnnamedField;
            let data = [
                // discriminant
                0, //
                // [u16; 12]
                0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0, 11, 0, 12,
            ];
            let (result, bytes_used) = deserialize::<UnnamedField>(&data);
            assert_eq!(bytes_used, data.len());
            assert_matches!(
                result,
                UnnamedField::Array(inner) if inner == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            );
        }

        #[test]
        fn enum_variant_types_unit() {
            use super::r#enum::VariantTypes;
            let data = [
                // discriminant
                0,
            ];
            let (result, bytes_used) = deserialize::<VariantTypes>(&data);
            assert_eq!(bytes_used, data.len());
            assert_matches!(result, VariantTypes::Unit);
        }

        #[test]
        fn enum_variant_types_newtype() {
            use super::r#enum::VariantTypes;
            let data = [
                // discriminant
                1, //
                // u8
                42,
            ];
            let (result, bytes_used) = deserialize::<VariantTypes>(&data);
            assert_eq!(bytes_used, data.len());
            assert_matches!(result, VariantTypes::Newtype(a) if a == 42);
        }

        #[test]
        fn enum_variant_types_unnamed() {
            use super::r#enum::VariantTypes;
            let data = [
                // discriminant
                2, //
                // u8
                42, //
                // u16
                0, 168,
            ];
            let (result, bytes_used) = deserialize::<VariantTypes>(&data);
            assert_eq!(bytes_used, data.len());
            assert_matches!(result, VariantTypes::Unnamed(a, b) if a == 42 && b == 168);
        }

        #[test]
        fn enum_variant_types_field() {
            use super::r#enum::VariantTypes;
            let data = [
                // discriminant
                3, //
                // u8
                42, //
                // u16
                0, 168,
            ];
            let (result, bytes_used) = deserialize::<VariantTypes>(&data);
            assert_eq!(bytes_used, data.len());
            assert_matches!(result, VariantTypes::Field{ a, b } if a == 42 && b == 168);
        }

        #[test]
        fn struct_named_field() {
            use super::r#struct::NamedField;
            let data = [
                // array
                0, 12, 0, 11, 0, 10, 0, 9, 0, 8, 0, 7, 0, 6, 0, 5, 0, 4, 0, 3, 0, 2, 0, 1, //
                // number
                0, 0, 0, 16,
            ];
            let (result, bytes_used) = deserialize::<NamedField>(&data);
            assert_eq!(bytes_used, data.len());
            assert_eq!(result.array, [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
            assert_eq!(result.number, 16);
        }

        #[test]
        fn struct_unnamed_field() {
            use super::r#struct::UnnamedField;
            let data = [
                // [u16; 12]
                0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0, 11, 0, 12, //
                // u32
                0, 0, 0, 16,
            ];
            let (result, bytes_used) = deserialize::<UnnamedField>(&data);
            assert_eq!(bytes_used, data.len());
            assert_eq!(result.0, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
            assert_eq!(result.1, 16);
        }
    }
}
