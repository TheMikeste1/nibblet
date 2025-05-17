#![expect(clippy::match_same_arms)]
use discrimin_ant_proc::discriminant;

#[derive(Debug)]
#[discriminant(u8)]
pub enum VariantTypes {
    Unit,
    Newtype(u8),
    Unnamed(u8, u16),
    Field { a: u8, b: u16 },
}
////////////////////////////////////////////////////////////////////////////////
struct _AssertDiscriminantable
where
    VariantTypes: discrimin_ant::Discriminantable;
impl serde::ser::Serialize for VariantTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let num_fields: usize = match self {
            Self::Unit => 0usize,
            Self::Newtype(..) => 1usize,
            Self::Unnamed(..) => 2usize,
            Self::Field { .. } => 2usize,
        }
        .checked_add(1usize)
        .unwrap_or_else(|| panic!("for there to be less than {} fields in an enum variant", usize::MAX));
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct(stringify!(VariantTypes), num_fields)?;
        let discriminant = discrimin_ant::Discriminantable::discriminant(self);
        s.serialize_field("__discriminant__", &discriminant)?;
        match self {
            Self::Unit => {}
            Self::Newtype(field_0) => {
                s.serialize_field(stringify!(field_0), &field_0)?;
            }
            Self::Unnamed(field_0, field_1) => {
                s.serialize_field(stringify!(field_0), &field_0)?;
                s.serialize_field(stringify!(field_1), &field_1)?;
            }
            Self::Field { a, b } => {
                s.serialize_field(stringify!(a), &a)?;
                s.serialize_field(stringify!(b), &b)?;
            }
        }
        s.end()
    }
}
////////////////////////////////////////////////////////////////////////////////
impl<'de> serde::de::Deserialize<'de> for VariantTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VariantTypes;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str(stringify!(enum VariantTypes))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                type DiscriminantType = <VariantTypes as discrimin_ant::Discriminantable>::Discriminant;
                let discriminant = serde::de::SeqAccess::next_element::<DiscriminantType>(&mut seq)?.ok_or_else(|| serde::de::Error::invalid_length(0usize, &"enum VariantTypes discriminant"))?;
                let variant = VariantTypes_::try_from(discriminant).map_err(|()| serde::de::Error::unknown_variant(&discriminant.to_string(), &["Unit", "Newtype", "Unnamed", "Field"]))?;
                match variant {
                    VariantTypes_::Unit => Ok(VariantTypes::Unit),
                    VariantTypes_::Newtype => {
                        let field_0 = serde::de::SeqAccess::next_element::<_>(&mut seq)?.ok_or_else(|| serde::de::Error::invalid_length(0usize, &"enum VariantTypes::Newtype with 1 element"))?;
                        Ok(VariantTypes::Newtype(field_0))
                    }
                    VariantTypes_::Unnamed => {
                        let field_0 = serde::de::SeqAccess::next_element::<_>(&mut seq)?.ok_or_else(|| serde::de::Error::invalid_length(0usize, &"enum VariantTypes::Unnamed with 2 elements"))?;
                        let field_1 = serde::de::SeqAccess::next_element::<_>(&mut seq)?.ok_or_else(|| serde::de::Error::invalid_length(1usize, &"enum VariantTypes::Unnamed with 2 elements"))?;
                        Ok(VariantTypes::Unnamed(field_0, field_1))
                    }
                    VariantTypes_::Field => {
                        let a = serde::de::SeqAccess::next_element::<_>(&mut seq)?.ok_or_else(|| serde::de::Error::invalid_length(0usize, &"enum VariantTypes::Field with 2 elements"))?;
                        let b = serde::de::SeqAccess::next_element::<_>(&mut seq)?.ok_or_else(|| serde::de::Error::invalid_length(1usize, &"enum VariantTypes::Field with 2 elements"))?;
                        Ok(VariantTypes::Field { a, b })
                    }
                }
            }
        }

        deserializer.deserialize_tuple_struct(stringify!(VariantTypes), usize::MAX, Visitor)
    }
}
