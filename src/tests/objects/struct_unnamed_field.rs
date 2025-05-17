pub struct UnnamedField(pub [u16; 12], pub u32);
////////////////////////////////////////////////////////////////////////////////
impl serde::ser::Serialize for UnnamedField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct(stringify!(UnnamedField), 2usize)?;
        self.0.iter().try_for_each(|elem| s.serialize_field(stringify!(self.0), elem))?;
        s.serialize_field(stringify!(self.1), &self.1)?;
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
                formatter.write_str(stringify!(struct UnnamedField))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut field_0 = [0; 12];
                for elem in field_0.as_mut() {
                    *elem = serde::de::SeqAccess::next_element::<_>(&mut seq)?.ok_or_else(|| serde::de::Error::invalid_length(0usize, &"struct UnnamedField with 2 elements"))?;
                }

                let field_1 = serde::de::SeqAccess::next_element::<_>(&mut seq)?.ok_or_else(|| serde::de::Error::invalid_length(1usize, &"struct UnnamedField with 2 elements"))?;
                Ok(UnnamedField(field_0, field_1))
            }
        }

        deserializer.deserialize_tuple_struct(stringify!(UnnamedField), usize::MAX, Visitor)
    }
}
