mod enum_named_field;
mod enum_unnamed_field;
mod enum_variant_types;
mod struct_named_field;
mod struct_unnamed_field;
mod test;

pub mod r#enum {
    pub use super::enum_named_field::*;
    pub use super::enum_unnamed_field::*;
    pub use super::enum_variant_types::*;
}

pub mod r#struct {
    pub use super::struct_named_field::*;
    pub use super::struct_unnamed_field::*;
}
