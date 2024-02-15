pub trait StringUtilsExt {
    fn abbreviate(str: &str, max_width: usize) -> String;
    fn abbreviate_with_offset(str: &str, offset: isize, max_width: usize) -> String;
    fn abbreviate_with_abbrev_marker(str: &str, abbrev_marker: &str, max_width: usize) -> String;
    fn abbreviate_with_full_opt(
        str: &str,
        abbrev_marker: &str,
        offset: isize,
        max_width: usize,
    ) -> String;
}

/// A String for a space character.
const SPACE: &'static str = " ";

/// The empty String {@code ""}.
const EMPTY: &'static str = "";

/// A String for linefeed LF ("\n").
const LF: &'static str = "\n";

/// A String for carriage return CR ("\r").
const CR: &'static str = "\r";

/// Represents a failed index search.
const INDEX_NOT_FOUND: usize = usize::MAX;

/// The maximum size to which the padding constant(s) can expand.
const PAD_LIMIT: usize = 8192;
