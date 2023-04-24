//! Some utility functions for the parser module

use super::ParseResult;
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};

/// Returns a Vec containing the positions of the character in the string
/// # Arguments
/// * `string` - The string to search
/// * `character` - The caracter to search for
pub fn char_occurrences(string: &str, character: char) -> Vec<usize> {
    let mut res: Vec<usize> = vec![];

    for (i, ch) in string.char_indices() {
        if ch == character {
            res.push(i);
        }
    }

    res
}

/// Parses an array in the following form from the supplied line: `[item][item]`
/// # Argument
/// * `line` - The line to parse
pub fn parse_array(line: &str) -> Result<Vec<String>, Error> {
    let mut res: Vec<String> = vec![];
    let mut buf = String::new();

    for character in line.chars() {
        match character {
            '[' => {
                buf.clear();
            }
            ']' => {
                res.push(buf.clone());
                buf.clear();
            }
            c => {
                buf.push(c);
            }
        }
    }

    if !buf.is_empty() {
        Err(Error::new(ErrorKind::UnexpectedEof, "While parsing array"))
    } else {
        Ok(res)
    }
}

/// A trait for convienient removal functions
pub trait Remove {
    /// Removes the first occurrence of the supplied character
    /// # Arguments
    /// * `character` - The character to remove
    fn remove_first(&mut self, character: char);

    /// Removes the last occurrence of the supplied character
    /// # Arguments
    /// * `character` - The character to remove
    fn remove_last(&mut self, character: char);
}

/// A trait for wrapping get operations in a io result for convenient use
pub trait GetExt {
    /// Returns an optional String for the supplied key.
    ///
    /// If the key does not exist, Ok(None) is returned.
    ///
    /// If the key is present, but the result is not a String, this returns Err().
    /// # Arguments
    /// * `key` - The key to search for
    fn get_str_opt(&self, key: &str) -> Result<Option<String>, Error>;

    /// Returns a String for the supplied key.
    ///
    /// If the key does not exist, this returns Err(NotFound).
    ///
    /// If the key is present, but the result is not a String, this returns Err(InvalidData).
    /// # Arguments
    /// * `key` - The key to search for
    fn get_str(&self, key: &str) -> Result<String, Error>;

    /// Returns an optional Vec for the supplied key.
    ///
    /// If the key does not exist, Ok(None) is returned.
    ///
    /// If the key is present, but the result is not a Vec, this returns Err(InvalidData).
    /// # Arguments
    /// * `key` - The key to search for
    fn get_vec_opt(&self, key: &str) -> Result<Option<Vec<String>>, Error>;

    /// Returns a Vec for the supplied key.
    ///
    /// If the key does not exist, this returns Err(NotFound).
    ///
    /// If the key is present, but the result is not a Vec, this returns Err(InvalidData).
    /// # Arguments
    /// * `key` - The key to search for
    fn get_vec(&self, key: &str) -> Result<Vec<String>, Error>;
}

/// Implement Remove for String
impl Remove for String {
    fn remove_last(&mut self, character: char) {
        let pos = self.chars().rev().position(|c| c == character);
        if let Some(pos) = pos {
            self.remove(self.len() - 1 - pos);
        }
    }

    fn remove_first(&mut self, character: char) {
        let pos = self.chars().position(|c| c == character);
        if let Some(pos) = pos {
            self.remove(pos);
        }
    }
}

/// Implement GetExt for HashMap<String, ParseResult>
impl GetExt for HashMap<String, ParseResult> {
    fn get_str_opt(&self, key: &str) -> Result<Option<String>, Error> {
        match self.get(key) {
            Some(value) => match value {
                ParseResult::String(v) => Ok(Some(v.clone())),
                ParseResult::Vec(_) => Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Expected String, got Vec for key '{}'", key),
                )),
            },
            None => Ok(None),
        }
    }

    fn get_str(&self, key: &str) -> Result<String, Error> {
        match self.get_str_opt(key)? {
            Some(v) => Ok(v),
            None => Err(Error::new(
                ErrorKind::NotFound,
                format!("Value with key '{}'", key),
            )),
        }
    }

    fn get_vec_opt(&self, key: &str) -> Result<Option<Vec<String>>, Error> {
        match self.get(key) {
            Some(value) => match value {
                ParseResult::String(_) => Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Expected Vec, got String for key '{}'", key),
                )),
                ParseResult::Vec(v) => Ok(Some(v.clone())),
            },
            None => Ok(None),
        }
    }

    fn get_vec(&self, key: &str) -> Result<Vec<String>, Error> {
        match self.get_vec_opt(key)? {
            Some(v) => Ok(v),
            None => Err(Error::new(
                ErrorKind::NotFound,
                format!("Value with key '{}'", key),
            )),
        }
    }
}
