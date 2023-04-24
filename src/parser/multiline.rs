use super::util::*;
use std::{io::Error, io::ErrorKind, slice::Iter};

/// Iterates over multiple lines to parse a Vec of Strings wrapped in `{}`
/// # Arguments
/// * `iter` - The iterator to iterate
/// * `start_line` - The start line
pub fn parse_multiline(iter: &mut Iter<&str>, start_line: &str) -> Result<Vec<String>, Error> {
    let mut lines: Vec<String> = vec![];
    let mut line = start_line.to_string();

    if char_occurrences(&line, '{').is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Tried to parse multiline, but no opening brace in start line",
        ));
    }

    //There is 1 opening brace we know of, and we remove it
    let mut ob = 1;
    let mut cb = 0;
    line.remove_first('{');

    //Now loop over the following lines
    loop {
        //Trim the line and append opening and closing braces
        line = line.trim().to_string();
        ob += char_occurrences(&line, '{').len();
        cb += char_occurrences(&line, '}').len();

        //If the block is finished (same openings as closings)
        if ob == cb {
            //Remove the last closing brace
            line.remove_last('}');

            //Push the line if possible and return
            if !line.is_empty() {
                lines.push(line);
            }
            return Ok(lines);
        }

        //Only push lines that aren't empty
        if !line.is_empty() {
            lines.push(line);
        }

        //Get the next line
        line = match iter.next() {
            Some(l) => l.to_string(),
            None => {
                return Err(Error::new(ErrorKind::UnexpectedEof, "In instruction block"));
            }
        }
    }
}
