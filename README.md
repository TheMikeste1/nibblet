# Nibblet

Nibblet is a byte-focussed alternative ser/de `derive`s.

---

[Serde](https://github.com/serde-rs/serde) is a fantastic library for serializing data into multiple formats, including
binary formats such as [bincode](https://github.com/bincode-org/bincode) and [postcard](https://github.com/jamesmunns/postcard).
However, the default serialize and deserialize `derive`s in Serde make cross-application/language use of these formats difficult.

Examples:
- bincode explicitly ignores the `repr` for an enum
- Serde [only passes the variant index](https://github.com/serde-rs/serde/blob/babafa54d283fb087fa94f50a2cf82fa9e582a7c/serde/src/ser/fmt.rs#L154-L162) when calling `serialize_*_variant` (the serialize methods for enums)
  - Applications and [ICDs](https://en.wikipedia.org/wiki/Interface_control_document) often require specific values, not indices
  - The index may change as develop continues, requiring updates to external code relying on the marshalled format
  - This makes it difficult to serialize the actual discriminant because that information is not passed
- [Constant-sized arrays larger than 32](https://github.com/serde-rs/serde/blob/babafa54d283fb087fa94f50a2cf82fa9e582a7c/serde/src/ser/impls.rs#L166-L171) are serialized as a sequence, just like dynamic sized arrays

This crate attempts to resolve these issues by providing alternative `Serialize` and `Deserialize` `derive`s.
These `derive`s are specifically designed to work with binary formats, and may not be compatible with other formats.

To use these alternative `derive`s, just use the `nibblet` versions instead of the `serde` versions:
 ```rust
 use discrimin_ant_proc::discriminant;
 use nibblet::Serialize;

 #[derive(Serialize)]
 #[discriminant(u8)] // Enums must provide a discriminant implementation
 enum FieldTypes {
     Unit,
     Newtype(u8),
     Unnamed(u8, u16),
     Field{ a: u8, b: u16 },
 }
```
