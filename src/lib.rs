//! This crate is used for hold this name before the development is finished.

// We still use the name string_utils which is used by others.
#[crate_name = "string-utils"]

mod string_utils;

pub use string_utils::StringUtilsExt;

