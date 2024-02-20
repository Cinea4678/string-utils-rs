//! This crate is used for hold this name before the development is finished.

// We still use the name string_utils which is used by others.
pub mod error;
#[crate_name = "string-utils"]
#[allow(unused)]
#[allow(dead_code)]
mod string_utils;

pub use string_utils::StringUtilsExt;
