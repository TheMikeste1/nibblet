//! Utilities for tests.

use proc_macro2::TokenStream;

/// Which part of an object is expected.
pub enum Expected {
    /// The serialize portion of an object is expected.
    Serialize,
    /// The deserialize portion of an object is expected.
    Deserialize,
}

/// Converts a str representation of the expected output into the actual form.
///
/// * `expected_str`: The string to expect, typically read from a file.
#[expect(clippy::unwrap_used)]
pub fn str_to_expected(expected_str: &str, expected: &Expected) -> String {
    let nth = match expected {
        Expected::Serialize => 1,
        Expected::Deserialize => 2,
    };
    let expected_str = expected_str.split("////////////////////////////////////////////////////////////////////////////////").map(String::from).nth(nth).unwrap();
    let expected = expected_str.parse::<TokenStream>().unwrap();
    let expected = expected.to_string();
    expected.replace("<'", "< '").replace(":<", ": <").replace(">:", "> :").replace("<&", "< &").replace("?;", "? ;").replace(">,", "> ,").replace("?.", "? .")
}
