pub struct NamedField {
    pub array: [u16; 12],
    pub number: u32,
}
////////////////////////////////////////////////////////////////////////////////
impl serde::ser::Serialize for NamedField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct(stringify!(NamedField), 2usize)?;
        self.array.iter().try_for_each(|elem| s.serialize_field(stringify!(self.array), elem))?;
        s.serialize_field(stringify!(self.number), &self.number)?;
        s.end()
    }
}
////////////////////////////////////////////////////////////////////////////////
impl<'de> serde::de::Deserialize<'de> for NamedField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NamedField;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str(stringify!(struct NamedField))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut array = [0; 12];
                for elem in array.as_mut() {
                    *elem = serde::de::SeqAccess::next_element::<_>(&mut seq)?.ok_or_else(|| serde::de::Error::invalid_length(0usize, &"struct NamedField with 2 elements"))?;
                }

                let number = serde::de::SeqAccess::next_element::<_>(&mut seq)?.ok_or_else(|| serde::de::Error::invalid_length(1usize, &"struct NamedField with 2 elements"))?;
                Ok(NamedField { array, number })
            }
        }

        deserializer.deserialize_tuple_struct(stringify!(NamedField), usize::MAX, Visitor)
    }
}
