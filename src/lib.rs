//! Includes procedural macros for nibblet.
mod de;
mod ser;
pub(crate) mod tests;

/// Derive `serde::ser::Serialize` using the enum's proper discriminant.
/// Uses `discrimin_ant::Discriminantable::discriminant` to identify the discriminant.
///
/// # Example
/// ```rust
/// use discrimin_ant_proc::discriminant;
/// use nibblet::Serialize;
///
/// #[derive(Serialize)]
/// #[discriminant(u8)]
/// enum FieldTypes {
///     Unit,
///     Newtype(u8),
///     Unnamed(u8, u16),
///     Field{ a: u8, b: u16 },
/// }
///```
#[proc_macro_derive(Serialize)]
pub fn derive_serialize(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    ser::derive_serialize_impl(item).into()
}

/// Derive `serde::de::Deserialize` using the enum's proper discriminant.
/// Uses `discrimin_ant::Discriminantable::discriminant` to identify the discriminant.
///
/// # Example
/// ```rust
/// use discrimin_ant_proc::discriminant;
/// use nibblet::Deserialize;
///
/// #[derive(Deserialize)]
/// #[discriminant(u8)]
/// enum FieldTypes {
///     Unit,
///     Newtype(u8),
///     Unnamed(u8, u16),
///     Field{ a: u8, b: u16 },
/// }
///```
#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    de::derive_deserialize_impl(item).into()
}
