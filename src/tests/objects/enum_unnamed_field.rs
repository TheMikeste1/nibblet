use discrimin_ant_proc::discriminant;

#[derive(Debug)]
#[discriminant(u8)]
pub enum UnnamedField {
    Array([u16; 12]),
}
////////////////////////////////////////////////////////////////////////////////
struct _AssertDiscriminantable
where
    UnnamedField: discrimin_ant::Discriminantable;
impl serde::ser::Serialize for UnnamedField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let num_fields: usize = match self {
            Self::Array(..) => 1usize,
        }
        .checked_add(1usize)
        .unwrap_or_else(|| panic!("for there to be less than {} fields in an enum variant", usize::MAX));
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct(stringify!(UnnamedField), num_fields)?;
        let discriminant = discrimin_ant::Discriminantable::discriminant(self);
        s.serialize_field("__discriminant__", &discriminant)?;
        match self {
            Self::Array(field_0) => {
                field_0.iter().try_for_each(|elem| s.serialize_field(stringify!(field_0), elem))?;
            }
        }
        s.end()
    }
}
////////////////////////////////////////////////////////////////////////////////
impl<'de> serde::de::Deserialize<'de> for UnnamedField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UnnamedField;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str(stringify!(enum UnnamedField))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                type DiscriminantType = <UnnamedField as discrimin_ant::Discriminantable>::Discriminant;
                let discriminant = serde::de::SeqAccess::next_element::<DiscriminantType>(&mut seq)?.ok_or_else(|| serde::de::Error::invalid_length(0usize, &"enum UnnamedField discriminant"))?;
                let variant = UnnamedField_::try_from(discriminant).map_err(|()| serde::de::Error::unknown_variant(&discriminant.to_string(), &["Array"]))?;
                match variant {
                    UnnamedField_::Array => {
                        let mut field_0 = [0; 12];
                        for elem in field_0.as_mut() {
                            *elem = serde::de::SeqAccess::next_element::<_>(&mut seq)?.ok_or_else(|| serde::de::Error::invalid_length(0usize, &"enum UnnamedField::Array with 1 element"))?;
                        }
                        Ok(UnnamedField::Array(field_0))
                    }
                }
            }
        }

        deserializer.deserialize_tuple_struct(stringify!(UnnamedField), usize::MAX, Visitor)
    }
}
