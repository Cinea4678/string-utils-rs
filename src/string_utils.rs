use crate::error::UtilsError::InvalidArgument;
use crate::error::UtilsResult;
use std::fmt::Display;
use std::ops::{Deref, Range, RangeBounds};

pub trait StringUtilsExt {
    fn abbreviate(&self, max_width: usize) -> UtilsResult<String>;
    fn abbreviate_with_abbrev_marker(
        &self,
        abbrev_marker: &str,
        max_width: usize,
    ) -> UtilsResult<String>;
    fn abbreviate_with_full_opt(
        &self,
        abbrev_marker: &str,
        offset: isize,
        max_width: usize,
    ) -> UtilsResult<String>;
    fn abbreviate_middle(&self, middle: &str, length: usize) -> UtilsResult<String>;
    fn abbreviate_with_offset(&self, offset: isize, max_width: usize) -> UtilsResult<String>;
    fn append_if_missing(&self, suffix: &str, ignore_case: bool) -> String;
    fn append_if_missing_one_of(
        &self,
        suffix: &str,
        suffixes: &[&str],
        ignore_case: bool,
    ) -> String;
    fn capitalize(&self) -> String;
    fn center(&self, size: usize) -> String;
    fn center_with(&self, size: usize, pad_str: &str) -> String;
    fn center_with_char(&self, size: usize, pad_char: &str) -> String;
    fn chomp(&self) -> String;
    fn chomp_specified(&self, separator: &str) -> String;
    fn chop(&self) -> String;
    fn compare(&self, str2: &str) -> i32;
    fn compare_ignore_case(&self, str2: &str) -> i32;
    fn contains(&self, search: &str) -> bool;
    fn contains_char(&self, search_char: char) -> bool;
    fn contains_any(&self, searches: &[&str]) -> bool;
    fn contains_any_char(&self, searches: &[char]) -> bool;
    fn contains_any_in(&self, searches_str: &str) -> bool;
    fn contains_any_ignore_case(&self, searches: &[&str]) -> bool;
    fn contains_any_with<T>(&self, searches: &[&str], test: T) -> bool
    where
        T: Fn(&str, &str) -> bool;
    fn contains_ignore_case(&self, search: &str) -> bool;
    fn contains_none(&self, searches: &[char]) -> bool;
    fn contains_none_in(&self, invalid_chars: &str) -> bool;
    fn contains_only(&self, valid: &[char]) -> bool;
    fn contains_only_in(&self, valid_chars: &str) -> bool;
    fn contains_whitespace(&self) -> bool;
    fn count_matches(&self, ch: char) -> u64;
    fn count_matches_str(&self, sub: &str) -> u64;
    fn default_if_blank(&self, default_str: &str) -> String;
    fn default_if_empty(&self, default_str: &str) -> String;
    fn default_string(&self) -> String;
    fn delete_whitespace(&self) -> String;
    fn difference(&self, str2: &str) -> String;
    fn digits(&self) -> String;
    fn end_with(&self, suffix: &str, ignore_case: bool) -> bool;
    fn end_with_any(&self, searches: &[&str]) -> bool;
    fn end_with_consider_case(&self, suffix: &str) -> bool;
    fn end_with_ignore_case(&self, suffix: &str) -> bool;
    fn equals(&self, str2: &str) -> bool;
    fn equals_any(&self, searches: &[&str]) -> bool;
    fn equals_any_ignore_case(&self, searches: &[&str]) -> bool;
    fn equals_ignore_case(&self, str2: &str) -> bool;
    fn if_blank<T>(&self, default_supplier: T) -> String
    where
        T: Fn() -> String;
    fn if_empty<T>(&self, default_supplier: T) -> String
    where
        T: Fn() -> String;
    fn index_of(&self, search: &str) -> Option<usize>;
    fn index_of_any(&self, searches: &[&str]) -> Option<usize>;
    fn index_of_any_but(&self, search_chars: &[char]) -> Option<usize>;
    fn index_of_any_but_in(&self, search_chars: &str) -> Option<usize>;
    fn index_of_any_char(&self, search_chars: &[char]) -> Option<usize>;
    fn index_of_any_in(&self, search_chars: &str) -> Option<usize>;
    fn index_of_char(&self, search_char: char) -> Option<usize>;
    fn index_of_char_starting_from(&self, search_char: char, start_pos: usize) -> Option<usize>;
    fn index_of_starting_from(&self, search: &str, start_pos: usize) -> Option<usize>;
    fn index_of_difference(&self, str2: &str) -> Option<usize>;
    fn index_of_ignore_case(&self, search: &str) -> Option<usize>;
    fn index_of_ignore_case_starting_from(&self, search: &str, start_pos: usize) -> Option<usize>;
    fn is_alpha(&self) -> bool;
    fn is_alphanumeric(&self) -> bool;
    fn is_alphanumeric_space(&self) -> bool;
    fn is_alpha_space(&self) -> bool;
    fn is_ascii_printable(&self) -> bool;
    fn is_blank(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn is_mixed_case(&self) -> bool;
    fn is_not_blank(&self) -> bool;
    fn is_not_empty(&self) -> bool;
    fn is_numeric(&self) -> bool;
    fn is_numeric_space(&self) -> bool;
    fn is_whitespace(&self) -> bool;
    fn last_index_of(&self, search: &str) -> Option<usize>;
    fn last_index_of_any(&self, searches: &[&str]) -> Option<usize>;
    fn last_index_of_char(&self, search_char: char) -> Option<usize>;
    fn last_index_of_char_starting_from(
        &self,
        search_char: char,
        start_pos: usize,
    ) -> Option<usize>;
    fn last_index_of_ignore_case(&self, search: &str) -> Option<usize>;
    fn last_index_of_ignore_case_starting_from(
        &self,
        search: &str,
        start_pos: usize,
    ) -> Option<usize>;
    fn last_index_of_starting_from(&self, search: &str, start_pos: usize) -> Option<usize>;
    fn last_ordinal_index_of(&self, search: &str, ordinal: usize) -> Option<usize>;
    fn left(&self, len: usize) -> String;
    fn left_pad(&self, size: usize) -> String;
    fn left_pad_with(&self, size: usize, pad_char: char) -> String;
    fn left_pad_with_str(&self, size: usize, pad_str: &str) -> String;
    fn lower_case(&self) -> String;
    fn mid(&self, pos: usize, len: usize) -> String;
    fn normalize_space(&self) -> String;
    fn ordinal_index_of(&self, search: &str, ordinal: usize) -> Option<usize>;
    fn overlay(&self, overlay: &str, start: usize, end: usize) -> String;
    fn prepend_if_missing(&self, prefix: &str, ignore_case: bool) -> String;
    fn prepend_if_missing_one_of(
        &self,
        prefix: &str,
        prefixes: &[&str],
        ignore_case: bool,
    ) -> String;
    fn remove(&self, remove: &str) -> String;
    fn remove_char(&self, remove: char) -> String;
    fn remove_all(&self, remove: &str) -> String;
    #[cfg(feature = "regex")]
    fn remove_all_regex(&self, regex: &str) -> String;
    fn remove_end(&self, remove: &str) -> String;
    fn remove_end_ignore_case(&self, remove: &str) -> String;
    fn remove_first(&self, remove: &str) -> String;
    #[cfg(feature = "regex")]
    fn remove_first_regex(&self, regex: &str) -> String;
    fn remove_ignore_case(&self, remove: &str) -> String;
    #[cfg(feature = "regex")]
    fn remove_pattern(&self, regex: &str) -> String;
    fn remove_start(&self, remove: &str) -> String;
    fn remove_start_char(&self, remove: char) -> String;
    fn remove_start_ignore_case(&self, remove: &str) -> String;
    fn repeat(&self, repeat: usize) -> String;
    fn replace_chars(&self, search: char, replace: char) -> String;
    fn replace_chars_in(&self, search_chars: &str, replace_chars: &str) -> String;
    fn replace_each(&self, search_list: &[&str], replace_list: &[&str]) -> UtilsResult<String>;
    fn replace_each_repeatedly(
        &self,
        search_list: &[&str],
        replace_list: &[&str],
        time_to_live: Option<usize>,
    ) -> UtilsResult<String>;
    fn replace_first(&self, search: &str, replace: &str) -> String;
    #[cfg(feature = "regex")]
    fn replace_first_regex(&self, search: &str, replace: &str) -> String;
    fn reverse(&self) -> String;
    fn reverse_delimited(&self, separator_char: char) -> String;
    fn right(&self, len: usize) -> String;
    fn right_pad(&self, size: usize) -> String;
    fn right_pad_with(&self, size: usize, pad_char: char) -> String;
    fn right_pad_with_str(&self, size: usize, pad_str: &str) -> String;
    fn rotate(&self, shift: isize) -> String;
    // fn split_by_character_type(&self) -> String;
    // fn split_by_character_type_and_camel_case(&self, camel_case: bool) -> String;
    fn starts_with(&self, prefix: &str, ignore_case: bool) -> bool;
    fn starts_with_any(&self, prefixes: &[&str]) -> bool;
    fn strip(&self) -> String;
    fn strip_in(&self, strip_chars: &str) -> String;
    fn strip_end(&self) -> String;
    fn strip_end_in(&self, strip_chars: &str) -> String;
    fn strip_start(&self) -> String;
    fn strip_start_in(&self, strip_chars: &str) -> String;
    fn substring(&self, range: Range<usize>) -> String;
    fn substring_after(&self, separator: char) -> String;
    fn substring_after_last(&self, separator: char) -> String;
    fn substring_after_last_str(&self, separator: &str) -> String;
    fn substring_after_str(&self, separator: &str) -> String;
    fn substring_before(&self, separator: char) -> String;
    fn substring_before_last(&self, separator: char) -> String;
    fn substring_before_last_str(&self, separator: &str) -> String;
    fn substring_before_str(&self, separator: &str) -> String;
    fn substring_between(&self, open: &str, close: &str) -> String;
    fn substring_between_tag(&self, tag: &str) -> String;
    fn swap_case(&self) -> String;
    fn trim(&self) -> String;
    fn truncate(&self, max_width: usize) -> String;
    fn truncate_with_offset(&self, offset: usize, max_width: usize) -> String;
    fn uncapitalize(&self) -> String;
    fn unwrap_from_char(&self, wrap_char: char) -> String;
    fn unwrap_from_str(&self, wrap_str: &str) -> String;
    fn upper_case(&self) -> String;
    fn wrap(&self, wrap_with: char) -> String;
    fn wrap_if_missing(&self, wrap_with: char) -> String;
    fn wrap_if_missing_str(&self, wrap_with: &str) -> String;
    fn wrap_with_str(&self, wrap_with: &str) -> String;
}

pub struct StringUtils;

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

#[allow(dead_code)]
#[allow(unused)]
impl StringUtils {
    pub fn first_non_blank<'a>(values: &[&'a str]) -> &'a str {
        todo!()
    }

    pub fn first_non_empty<'a>(values: &[&'a str]) -> &'a str {
        todo!()
    }

    pub fn get_common_prefix(str: &[&str]) -> String {
        todo!()
    }

    pub fn index_of_difference(values: &[&str]) -> Option<usize> {
        todo!()
    }

    pub fn is_all_blank(values: &[&str]) -> bool {
        todo!()
    }

    pub fn is_all_empty(values: &[&str]) -> bool {
        todo!()
    }

    pub fn is_all_lower_case(values: &[&str]) -> bool {
        todo!()
    }

    pub fn is_all_upper_case(values: &[&str]) -> bool {
        todo!()
    }

    pub fn is_any_blank(values: &[&str]) -> bool {
        todo!()
    }

    pub fn is_any_empty(values: &[&str]) -> bool {
        todo!()
    }

    pub fn is_none_blank(values: &[&str]) -> bool {
        todo!()
    }

    pub fn is_none_empty(values: &[&str]) -> bool {
        todo!()
    }

    pub fn join<T>(array: &[&T], separator: char) -> String
    where
        T: Display,
    {
        todo!()
    }

    pub fn join_with_str<T>(array: &[&T], separator: &str) -> String
    where
        T: Display,
    {
        todo!()
    }

    pub fn join_in_range<T>(
        array: &[&T],
        delimiter: char,
        start_index: usize,
        end_index: usize,
    ) -> String
    where
        T: Display,
    {
        todo!()
    }

    pub fn join_in_range_with_str<T>(
        array: &[&T],
        separator: &str,
        start_index: usize,
        end_index: usize,
    ) -> String
    where
        T: Display,
    {
        todo!()
    }

    pub fn strip_all(values: &[&str]) -> Vec<String> {
        todo!()
    }

    pub fn strip_all_in(values: &[&str], strip_chars: &str) -> Vec<String> {
        todo!()
    }
}

impl<'a> StringUtilsExt for &'a str {
    fn abbreviate(&self, max_width: usize) -> UtilsResult<String> {
        self.abbreviate_with_full_opt("...", 0, max_width)
    }

    fn abbreviate_with_abbrev_marker(
        &self,
        abbrev_marker: &str,
        max_width: usize,
    ) -> UtilsResult<String> {
        self.abbreviate_with_full_opt(abbrev_marker, 0, max_width)
    }

    fn abbreviate_with_full_opt(
        &self,
        abbrev_marker: &str,
        mut offset: isize,
        max_width: usize,
    ) -> UtilsResult<String> {
        if self.is_not_empty() && abbrev_marker == EMPTY && max_width > 0 {
            return Ok(self.substring(0..max_width));
        }

        if StringUtils::is_any_empty(&[self, abbrev_marker]) {
            return Ok(String::from(*self));
        }

        let abbrev_marker_length = abbrev_marker.len();
        let min_abbrev_width = abbrev_marker_length + 1;
        let min_abbrev_width_offset = abbrev_marker_length + abbrev_marker_length + 1;

        if max_width < min_abbrev_width {
            return Err(InvalidArgument(format!(
                "Minimum abbreviation width is {}",
                min_abbrev_width
            )));
        }

        let str_len = self.len() as isize;
        if str_len <= max_width as isize {
            return Ok(String::from(*self));
        }
        if offset > str_len {
            offset = str_len;
        }
        if str_len - offset < (max_width - abbrev_marker_length) as isize {
            offset = str_len - (max_width - abbrev_marker_length) as isize
        }
        if offset <= (abbrev_marker_length + 1) as isize {
            return Ok(self.substring(0..(max_width - abbrev_marker_length)) + abbrev_marker);
        }
        if max_width < min_abbrev_width_offset {
            return Err(InvalidArgument(format!(
                "Minimum abbreviation width with offset is {}",
                min_abbrev_width_offset
            )));
        }
        if (offset + (max_width - abbrev_marker_length) as isize) < str_len {
            return Ok(String::from(abbrev_marker)
                + &self
                    .substring(offset as usize..self.len())
                    .as_str()
                    .abbreviate_with_full_opt(
                        abbrev_marker,
                        0,
                        max_width - abbrev_marker_length,
                    )?);
        }

        return Ok(String::from(abbrev_marker)
            + &self.substring(
                (str_len - (max_width - abbrev_marker_length) as isize) as usize..self.len(),
            ));
    }

    fn abbreviate_middle(&self, middle: &str, length: usize) -> UtilsResult<String> {
        todo!()
    }

    fn abbreviate_with_offset(&self, offset: isize, max_width: usize) -> UtilsResult<String> {
        todo!()
    }

    fn append_if_missing(&self, suffix: &str, ignore_case: bool) -> String {
        todo!()
    }

    fn append_if_missing_one_of(
        &self,
        suffix: &str,
        suffixes: &[&str],
        ignore_case: bool,
    ) -> String {
        todo!()
    }

    fn capitalize(&self) -> String {
        todo!()
    }

    fn center(&self, size: usize) -> String {
        todo!()
    }

    fn center_with(&self, size: usize, pad_str: &str) -> String {
        todo!()
    }

    fn center_with_char(&self, size: usize, pad_char: &str) -> String {
        todo!()
    }

    fn chomp(&self) -> String {
        todo!()
    }

    fn chomp_specified(&self, separator: &str) -> String {
        todo!()
    }

    fn chop(&self) -> String {
        todo!()
    }

    fn compare(&self, str2: &str) -> i32 {
        todo!()
    }

    fn compare_ignore_case(&self, str2: &str) -> i32 {
        todo!()
    }

    fn contains(&self, search: &str) -> bool {
        todo!()
    }

    fn contains_char(&self, search_char: char) -> bool {
        todo!()
    }

    fn contains_any(&self, searches: &[&str]) -> bool {
        todo!()
    }

    fn contains_any_char(&self, searches: &[char]) -> bool {
        todo!()
    }

    fn contains_any_in(&self, searches_str: &str) -> bool {
        todo!()
    }

    fn contains_any_ignore_case(&self, searches: &[&str]) -> bool {
        todo!()
    }

    fn contains_any_with<T>(&self, searches: &[&str], test: T) -> bool
    where
        T: Fn(&str, &str) -> bool,
    {
        todo!()
    }

    fn contains_ignore_case(&self, search: &str) -> bool {
        todo!()
    }

    fn contains_none(&self, searches: &[char]) -> bool {
        todo!()
    }

    fn contains_none_in(&self, invalid_chars: &str) -> bool {
        todo!()
    }

    fn contains_only(&self, valid: &[char]) -> bool {
        todo!()
    }

    fn contains_only_in(&self, valid_chars: &str) -> bool {
        todo!()
    }

    fn contains_whitespace(&self) -> bool {
        todo!()
    }

    fn count_matches(&self, ch: char) -> u64 {
        todo!()
    }

    fn count_matches_str(&self, sub: &str) -> u64 {
        todo!()
    }

    fn default_if_blank(&self, default_str: &str) -> String {
        todo!()
    }

    fn default_if_empty(&self, default_str: &str) -> String {
        todo!()
    }

    fn default_string(&self) -> String {
        todo!()
    }

    fn delete_whitespace(&self) -> String {
        todo!()
    }

    fn difference(&self, str2: &str) -> String {
        todo!()
    }

    fn digits(&self) -> String {
        todo!()
    }

    fn end_with(&self, suffix: &str, ignore_case: bool) -> bool {
        todo!()
    }

    fn end_with_any(&self, searches: &[&str]) -> bool {
        todo!()
    }

    fn end_with_consider_case(&self, suffix: &str) -> bool {
        todo!()
    }

    fn end_with_ignore_case(&self, suffix: &str) -> bool {
        todo!()
    }

    fn equals(&self, str2: &str) -> bool {
        todo!()
    }

    fn equals_any(&self, searches: &[&str]) -> bool {
        todo!()
    }

    fn equals_any_ignore_case(&self, searches: &[&str]) -> bool {
        todo!()
    }

    fn equals_ignore_case(&self, str2: &str) -> bool {
        todo!()
    }

    fn if_blank<T>(&self, default_supplier: T) -> String
    where
        T: Fn() -> String,
    {
        todo!()
    }

    fn if_empty<T>(&self, default_supplier: T) -> String
    where
        T: Fn() -> String,
    {
        todo!()
    }

    fn index_of(&self, search: &str) -> Option<usize> {
        todo!()
    }

    fn index_of_any(&self, searches: &[&str]) -> Option<usize> {
        todo!()
    }

    fn index_of_any_but(&self, search_chars: &[char]) -> Option<usize> {
        todo!()
    }

    fn index_of_any_but_in(&self, search_chars: &str) -> Option<usize> {
        todo!()
    }

    fn index_of_any_char(&self, search_chars: &[char]) -> Option<usize> {
        todo!()
    }

    fn index_of_any_in(&self, search_chars: &str) -> Option<usize> {
        todo!()
    }

    fn index_of_char(&self, search_char: char) -> Option<usize> {
        todo!()
    }

    fn index_of_char_starting_from(&self, search_char: char, start_pos: usize) -> Option<usize> {
        todo!()
    }

    fn index_of_starting_from(&self, search: &str, start_pos: usize) -> Option<usize> {
        todo!()
    }

    fn index_of_difference(&self, str2: &str) -> Option<usize> {
        todo!()
    }

    fn index_of_ignore_case(&self, search: &str) -> Option<usize> {
        todo!()
    }

    fn index_of_ignore_case_starting_from(&self, search: &str, start_pos: usize) -> Option<usize> {
        todo!()
    }

    fn is_alpha(&self) -> bool {
        todo!()
    }

    fn is_alphanumeric(&self) -> bool {
        todo!()
    }

    fn is_alphanumeric_space(&self) -> bool {
        todo!()
    }

    fn is_alpha_space(&self) -> bool {
        todo!()
    }

    fn is_ascii_printable(&self) -> bool {
        todo!()
    }

    fn is_blank(&self) -> bool {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn is_mixed_case(&self) -> bool {
        todo!()
    }

    fn is_not_blank(&self) -> bool {
        todo!()
    }

    fn is_not_empty(&self) -> bool {
        todo!()
    }

    fn is_numeric(&self) -> bool {
        todo!()
    }

    fn is_numeric_space(&self) -> bool {
        todo!()
    }

    fn is_whitespace(&self) -> bool {
        todo!()
    }

    fn last_index_of(&self, search: &str) -> Option<usize> {
        todo!()
    }

    fn last_index_of_any(&self, searches: &[&str]) -> Option<usize> {
        todo!()
    }

    fn last_index_of_char(&self, search_char: char) -> Option<usize> {
        todo!()
    }

    fn last_index_of_char_starting_from(
        &self,
        search_char: char,
        start_pos: usize,
    ) -> Option<usize> {
        todo!()
    }

    fn last_index_of_ignore_case(&self, search: &str) -> Option<usize> {
        todo!()
    }

    fn last_index_of_ignore_case_starting_from(
        &self,
        search: &str,
        start_pos: usize,
    ) -> Option<usize> {
        todo!()
    }

    fn last_index_of_starting_from(&self, search: &str, start_pos: usize) -> Option<usize> {
        todo!()
    }

    fn last_ordinal_index_of(&self, search: &str, ordinal: usize) -> Option<usize> {
        todo!()
    }

    fn left(&self, len: usize) -> String {
        todo!()
    }

    fn left_pad(&self, size: usize) -> String {
        todo!()
    }

    fn left_pad_with(&self, size: usize, pad_char: char) -> String {
        todo!()
    }

    fn left_pad_with_str(&self, size: usize, pad_str: &str) -> String {
        todo!()
    }

    fn lower_case(&self) -> String {
        todo!()
    }

    fn mid(&self, pos: usize, len: usize) -> String {
        todo!()
    }

    fn normalize_space(&self) -> String {
        todo!()
    }

    fn ordinal_index_of(&self, search: &str, ordinal: usize) -> Option<usize> {
        todo!()
    }

    fn overlay(&self, overlay: &str, start: usize, end: usize) -> String {
        todo!()
    }

    fn prepend_if_missing(&self, prefix: &str, ignore_case: bool) -> String {
        todo!()
    }

    fn prepend_if_missing_one_of(
        &self,
        prefix: &str,
        prefixes: &[&str],
        ignore_case: bool,
    ) -> String {
        todo!()
    }

    fn remove(&self, remove: &str) -> String {
        todo!()
    }

    fn remove_char(&self, remove: char) -> String {
        todo!()
    }

    fn remove_all(&self, remove: &str) -> String {
        todo!()
    }

    fn remove_end(&self, remove: &str) -> String {
        todo!()
    }

    fn remove_end_ignore_case(&self, remove: &str) -> String {
        todo!()
    }

    fn remove_first(&self, remove: &str) -> String {
        todo!()
    }

    fn remove_ignore_case(&self, remove: &str) -> String {
        todo!()
    }

    fn remove_start(&self, remove: &str) -> String {
        todo!()
    }

    fn remove_start_char(&self, remove: char) -> String {
        todo!()
    }

    fn remove_start_ignore_case(&self, remove: &str) -> String {
        todo!()
    }

    fn repeat(&self, repeat: usize) -> String {
        todo!()
    }

    fn replace_chars(&self, search: char, replace: char) -> String {
        todo!()
    }

    fn replace_chars_in(&self, search_chars: &str, replace_chars: &str) -> String {
        todo!()
    }

    fn replace_each(&self, search_list: &[&str], replace_list: &[&str]) -> UtilsResult<String> {
        todo!()
    }

    fn replace_each_repeatedly(
        &self,
        search_list: &[&str],
        replace_list: &[&str],
        time_to_live: Option<usize>,
    ) -> UtilsResult<String> {
        todo!()
    }

    fn replace_first(&self, search: &str, replace: &str) -> String {
        todo!()
    }

    fn reverse(&self) -> String {
        todo!()
    }

    fn reverse_delimited(&self, separator_char: char) -> String {
        todo!()
    }

    fn right(&self, len: usize) -> String {
        todo!()
    }

    fn right_pad(&self, size: usize) -> String {
        todo!()
    }

    fn right_pad_with(&self, size: usize, pad_char: char) -> String {
        todo!()
    }

    fn right_pad_with_str(&self, size: usize, pad_str: &str) -> String {
        todo!()
    }

    fn rotate(&self, shift: isize) -> String {
        todo!()
    }

    fn starts_with(&self, prefix: &str, ignore_case: bool) -> bool {
        todo!()
    }

    fn starts_with_any(&self, prefixes: &[&str]) -> bool {
        todo!()
    }

    fn strip(&self) -> String {
        todo!()
    }

    fn strip_in(&self, strip_chars: &str) -> String {
        todo!()
    }

    fn strip_end(&self) -> String {
        todo!()
    }

    fn strip_end_in(&self, strip_chars: &str) -> String {
        todo!()
    }

    fn strip_start(&self) -> String {
        todo!()
    }

    fn strip_start_in(&self, strip_chars: &str) -> String {
        todo!()
    }

    fn substring(&self, range: Range<usize>) -> String {
        todo!()
    }

    fn substring_after(&self, separator: char) -> String {
        todo!()
    }

    fn substring_after_last(&self, separator: char) -> String {
        todo!()
    }

    fn substring_after_last_str(&self, separator: &str) -> String {
        todo!()
    }

    fn substring_after_str(&self, separator: &str) -> String {
        todo!()
    }

    fn substring_before(&self, separator: char) -> String {
        todo!()
    }

    fn substring_before_last(&self, separator: char) -> String {
        todo!()
    }

    fn substring_before_last_str(&self, separator: &str) -> String {
        todo!()
    }

    fn substring_before_str(&self, separator: &str) -> String {
        todo!()
    }

    fn substring_between(&self, open: &str, close: &str) -> String {
        todo!()
    }

    fn substring_between_tag(&self, tag: &str) -> String {
        todo!()
    }

    fn swap_case(&self) -> String {
        todo!()
    }

    fn trim(&self) -> String {
        todo!()
    }

    fn truncate(&self, max_width: usize) -> String {
        todo!()
    }

    fn truncate_with_offset(&self, offset: usize, max_width: usize) -> String {
        todo!()
    }

    fn uncapitalize(&self) -> String {
        todo!()
    }

    fn unwrap_from_char(&self, wrap_char: char) -> String {
        todo!()
    }

    fn unwrap_from_str(&self, wrap_str: &str) -> String {
        todo!()
    }

    fn upper_case(&self) -> String {
        todo!()
    }

    fn wrap(&self, wrap_with: char) -> String {
        todo!()
    }

    fn wrap_if_missing(&self, wrap_with: char) -> String {
        todo!()
    }

    fn wrap_if_missing_str(&self, wrap_with: &str) -> String {
        todo!()
    }

    fn wrap_with_str(&self, wrap_with: &str) -> String {
        todo!()
    }
}

impl StringUtilsExt for String {
    fn abbreviate(&self, max_width: usize) -> UtilsResult<String> {
        self.as_str().abbreviate(max_width)
    }

    fn abbreviate_with_abbrev_marker(
        &self,
        abbrev_marker: &str,
        max_width: usize,
    ) -> UtilsResult<String> {
        self.as_str()
            .abbreviate_with_abbrev_marker(abbrev_marker, max_width)
    }

    fn abbreviate_with_full_opt(
        &self,
        abbrev_marker: &str,
        offset: isize,
        max_width: usize,
    ) -> UtilsResult<String> {
        self.as_str()
            .abbreviate_with_full_opt(abbrev_marker, offset, max_width)
    }

    fn abbreviate_middle(&self, middle: &str, length: usize) -> UtilsResult<String> {
        todo!()
    }

    fn abbreviate_with_offset(&self, offset: isize, max_width: usize) -> UtilsResult<String> {
        todo!()
    }

    fn append_if_missing(&self, suffix: &str, ignore_case: bool) -> String {
        todo!()
    }

    fn append_if_missing_one_of(
        &self,
        suffix: &str,
        suffixes: &[&str],
        ignore_case: bool,
    ) -> String {
        todo!()
    }

    fn capitalize(&self) -> String {
        todo!()
    }

    fn center(&self, size: usize) -> String {
        todo!()
    }

    fn center_with(&self, size: usize, pad_str: &str) -> String {
        todo!()
    }

    fn center_with_char(&self, size: usize, pad_char: &str) -> String {
        todo!()
    }

    fn chomp(&self) -> String {
        todo!()
    }

    fn chomp_specified(&self, separator: &str) -> String {
        todo!()
    }

    fn chop(&self) -> String {
        todo!()
    }

    fn compare(&self, str2: &str) -> i32 {
        todo!()
    }

    fn compare_ignore_case(&self, str2: &str) -> i32 {
        todo!()
    }

    fn contains(&self, search: &str) -> bool {
        todo!()
    }

    fn contains_char(&self, search_char: char) -> bool {
        todo!()
    }

    fn contains_any(&self, searches: &[&str]) -> bool {
        todo!()
    }

    fn contains_any_char(&self, searches: &[char]) -> bool {
        todo!()
    }

    fn contains_any_in(&self, searches_str: &str) -> bool {
        todo!()
    }

    fn contains_any_ignore_case(&self, searches: &[&str]) -> bool {
        todo!()
    }

    fn contains_any_with<T>(&self, searches: &[&str], test: T) -> bool
    where
        T: Fn(&str, &str) -> bool,
    {
        todo!()
    }

    fn contains_ignore_case(&self, search: &str) -> bool {
        todo!()
    }

    fn contains_none(&self, searches: &[char]) -> bool {
        todo!()
    }

    fn contains_none_in(&self, invalid_chars: &str) -> bool {
        todo!()
    }

    fn contains_only(&self, valid: &[char]) -> bool {
        todo!()
    }

    fn contains_only_in(&self, valid_chars: &str) -> bool {
        todo!()
    }

    fn contains_whitespace(&self) -> bool {
        todo!()
    }

    fn count_matches(&self, ch: char) -> u64 {
        todo!()
    }

    fn count_matches_str(&self, sub: &str) -> u64 {
        todo!()
    }

    fn default_if_blank(&self, default_str: &str) -> String {
        todo!()
    }

    fn default_if_empty(&self, default_str: &str) -> String {
        todo!()
    }

    fn default_string(&self) -> String {
        todo!()
    }

    fn delete_whitespace(&self) -> String {
        todo!()
    }

    fn difference(&self, str2: &str) -> String {
        todo!()
    }

    fn digits(&self) -> String {
        todo!()
    }

    fn end_with(&self, suffix: &str, ignore_case: bool) -> bool {
        todo!()
    }

    fn end_with_any(&self, searches: &[&str]) -> bool {
        todo!()
    }

    fn end_with_consider_case(&self, suffix: &str) -> bool {
        todo!()
    }

    fn end_with_ignore_case(&self, suffix: &str) -> bool {
        todo!()
    }

    fn equals(&self, str2: &str) -> bool {
        todo!()
    }

    fn equals_any(&self, searches: &[&str]) -> bool {
        todo!()
    }

    fn equals_any_ignore_case(&self, searches: &[&str]) -> bool {
        todo!()
    }

    fn equals_ignore_case(&self, str2: &str) -> bool {
        todo!()
    }

    fn if_blank<T>(&self, default_supplier: T) -> String
    where
        T: Fn() -> String,
    {
        todo!()
    }

    fn if_empty<T>(&self, default_supplier: T) -> String
    where
        T: Fn() -> String,
    {
        todo!()
    }

    fn index_of(&self, search: &str) -> Option<usize> {
        todo!()
    }

    fn index_of_any(&self, searches: &[&str]) -> Option<usize> {
        todo!()
    }

    fn index_of_any_but(&self, search_chars: &[char]) -> Option<usize> {
        todo!()
    }

    fn index_of_any_but_in(&self, search_chars: &str) -> Option<usize> {
        todo!()
    }

    fn index_of_any_char(&self, search_chars: &[char]) -> Option<usize> {
        todo!()
    }

    fn index_of_any_in(&self, search_chars: &str) -> Option<usize> {
        todo!()
    }

    fn index_of_char(&self, search_char: char) -> Option<usize> {
        todo!()
    }

    fn index_of_char_starting_from(&self, search_char: char, start_pos: usize) -> Option<usize> {
        todo!()
    }

    fn index_of_starting_from(&self, search: &str, start_pos: usize) -> Option<usize> {
        todo!()
    }

    fn index_of_difference(&self, str2: &str) -> Option<usize> {
        todo!()
    }

    fn index_of_ignore_case(&self, search: &str) -> Option<usize> {
        todo!()
    }

    fn index_of_ignore_case_starting_from(&self, search: &str, start_pos: usize) -> Option<usize> {
        todo!()
    }

    fn is_alpha(&self) -> bool {
        todo!()
    }

    fn is_alphanumeric(&self) -> bool {
        todo!()
    }

    fn is_alphanumeric_space(&self) -> bool {
        todo!()
    }

    fn is_alpha_space(&self) -> bool {
        todo!()
    }

    fn is_ascii_printable(&self) -> bool {
        todo!()
    }

    fn is_blank(&self) -> bool {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn is_mixed_case(&self) -> bool {
        todo!()
    }

    fn is_not_blank(&self) -> bool {
        todo!()
    }

    fn is_not_empty(&self) -> bool {
        todo!()
    }

    fn is_numeric(&self) -> bool {
        todo!()
    }

    fn is_numeric_space(&self) -> bool {
        todo!()
    }

    fn is_whitespace(&self) -> bool {
        todo!()
    }

    fn last_index_of(&self, search: &str) -> Option<usize> {
        todo!()
    }

    fn last_index_of_any(&self, searches: &[&str]) -> Option<usize> {
        todo!()
    }

    fn last_index_of_char(&self, search_char: char) -> Option<usize> {
        todo!()
    }

    fn last_index_of_char_starting_from(
        &self,
        search_char: char,
        start_pos: usize,
    ) -> Option<usize> {
        todo!()
    }

    fn last_index_of_ignore_case(&self, search: &str) -> Option<usize> {
        todo!()
    }

    fn last_index_of_ignore_case_starting_from(
        &self,
        search: &str,
        start_pos: usize,
    ) -> Option<usize> {
        todo!()
    }

    fn last_index_of_starting_from(&self, search: &str, start_pos: usize) -> Option<usize> {
        todo!()
    }

    fn last_ordinal_index_of(&self, search: &str, ordinal: usize) -> Option<usize> {
        todo!()
    }

    fn left(&self, len: usize) -> String {
        todo!()
    }

    fn left_pad(&self, size: usize) -> String {
        todo!()
    }

    fn left_pad_with(&self, size: usize, pad_char: char) -> String {
        todo!()
    }

    fn left_pad_with_str(&self, size: usize, pad_str: &str) -> String {
        todo!()
    }

    fn lower_case(&self) -> String {
        todo!()
    }

    fn mid(&self, pos: usize, len: usize) -> String {
        todo!()
    }

    fn normalize_space(&self) -> String {
        todo!()
    }

    fn ordinal_index_of(&self, search: &str, ordinal: usize) -> Option<usize> {
        todo!()
    }

    fn overlay(&self, overlay: &str, start: usize, end: usize) -> String {
        todo!()
    }

    fn prepend_if_missing(&self, prefix: &str, ignore_case: bool) -> String {
        todo!()
    }

    fn prepend_if_missing_one_of(
        &self,
        prefix: &str,
        prefixes: &[&str],
        ignore_case: bool,
    ) -> String {
        todo!()
    }

    fn remove(&self, remove: &str) -> String {
        todo!()
    }

    fn remove_char(&self, remove: char) -> String {
        todo!()
    }

    fn remove_all(&self, remove: &str) -> String {
        todo!()
    }

    fn remove_end(&self, remove: &str) -> String {
        todo!()
    }

    fn remove_end_ignore_case(&self, remove: &str) -> String {
        todo!()
    }

    fn remove_first(&self, remove: &str) -> String {
        todo!()
    }

    fn remove_ignore_case(&self, remove: &str) -> String {
        todo!()
    }

    fn remove_start(&self, remove: &str) -> String {
        todo!()
    }

    fn remove_start_char(&self, remove: char) -> String {
        todo!()
    }

    fn remove_start_ignore_case(&self, remove: &str) -> String {
        todo!()
    }

    fn repeat(&self, repeat: usize) -> String {
        todo!()
    }

    fn replace_chars(&self, search: char, replace: char) -> String {
        todo!()
    }

    fn replace_chars_in(&self, search_chars: &str, replace_chars: &str) -> String {
        todo!()
    }

    fn replace_each(&self, search_list: &[&str], replace_list: &[&str]) -> UtilsResult<String> {
        todo!()
    }

    fn replace_each_repeatedly(
        &self,
        search_list: &[&str],
        replace_list: &[&str],
        time_to_live: Option<usize>,
    ) -> UtilsResult<String> {
        todo!()
    }

    fn replace_first(&self, search: &str, replace: &str) -> String {
        todo!()
    }

    fn reverse(&self) -> String {
        todo!()
    }

    fn reverse_delimited(&self, separator_char: char) -> String {
        todo!()
    }

    fn right(&self, len: usize) -> String {
        todo!()
    }

    fn right_pad(&self, size: usize) -> String {
        todo!()
    }

    fn right_pad_with(&self, size: usize, pad_char: char) -> String {
        todo!()
    }

    fn right_pad_with_str(&self, size: usize, pad_str: &str) -> String {
        todo!()
    }

    fn rotate(&self, shift: isize) -> String {
        todo!()
    }

    fn starts_with(&self, prefix: &str, ignore_case: bool) -> bool {
        todo!()
    }

    fn starts_with_any(&self, prefixes: &[&str]) -> bool {
        todo!()
    }

    fn strip(&self) -> String {
        todo!()
    }

    fn strip_in(&self, strip_chars: &str) -> String {
        todo!()
    }

    fn strip_end(&self) -> String {
        todo!()
    }

    fn strip_end_in(&self, strip_chars: &str) -> String {
        todo!()
    }

    fn strip_start(&self) -> String {
        todo!()
    }

    fn strip_start_in(&self, strip_chars: &str) -> String {
        todo!()
    }

    fn substring(&self, range: Range<usize>) -> String {
        todo!()
    }

    fn substring_after(&self, separator: char) -> String {
        todo!()
    }

    fn substring_after_last(&self, separator: char) -> String {
        todo!()
    }

    fn substring_after_last_str(&self, separator: &str) -> String {
        todo!()
    }

    fn substring_after_str(&self, separator: &str) -> String {
        todo!()
    }

    fn substring_before(&self, separator: char) -> String {
        todo!()
    }

    fn substring_before_last(&self, separator: char) -> String {
        todo!()
    }

    fn substring_before_last_str(&self, separator: &str) -> String {
        todo!()
    }

    fn substring_before_str(&self, separator: &str) -> String {
        todo!()
    }

    fn substring_between(&self, open: &str, close: &str) -> String {
        todo!()
    }

    fn substring_between_tag(&self, tag: &str) -> String {
        todo!()
    }

    fn swap_case(&self) -> String {
        todo!()
    }

    fn trim(&self) -> String {
        todo!()
    }

    fn truncate(&self, max_width: usize) -> String {
        todo!()
    }

    fn truncate_with_offset(&self, offset: usize, max_width: usize) -> String {
        todo!()
    }

    fn uncapitalize(&self) -> String {
        todo!()
    }

    fn unwrap_from_char(&self, wrap_char: char) -> String {
        todo!()
    }

    fn unwrap_from_str(&self, wrap_str: &str) -> String {
        todo!()
    }

    fn upper_case(&self) -> String {
        todo!()
    }

    fn wrap(&self, wrap_with: char) -> String {
        todo!()
    }

    fn wrap_if_missing(&self, wrap_with: char) -> String {
        todo!()
    }

    fn wrap_if_missing_str(&self, wrap_with: &str) -> String {
        todo!()
    }

    fn wrap_with_str(&self, wrap_with: &str) -> String {
        todo!()
    }
}
