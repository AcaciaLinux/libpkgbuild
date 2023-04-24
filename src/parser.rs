mod multiline;
pub mod util;

use crate::PackageBuild;
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Read};
use util::GetExt;

///A parse result can either be a normal string or a vector of strings
#[derive(Debug)]
enum ParseResult {
    String(String),
    Vec(Vec<String>),
}

///Parses the packagebuild contents line by line
/// # Arguments
/// * `lines` - A Vec containing all the lines
fn parse_pkgbuild(lines: Vec<&str>) -> Result<HashMap<String, ParseResult>, Error> {
    //Create the return and an iterator over the lines
    let mut map: HashMap<String, ParseResult> = HashMap::new();
    let mut iter = lines.iter();

    //Iterate over the lines
    while let Some(line) = iter.next() {
        //If we can split the line by `=`
        if let Some(v) = line.split_once('=') {
            //Trim the key and the contents
            let key = v.0.trim().to_string();
            let data = v.1.trim().to_string();
            let ob = util::char_occurrences(&data, '{');

            //Ignore empty data
            if data.is_empty() {
                continue;
            }

            //If there is an array to parse
            if data.starts_with('[') {
                let res = util::parse_array(&data)?;
                if !res.is_empty() {
                    map.insert(key, ParseResult::Vec(res));
                }
            }
            // If there is no opening brace, use the data as-is
            else if ob.is_empty() {
                map.insert(key, ParseResult::String(v.1.trim().to_string()));
            }
            //Else, parse multiline
            else {
                let multiline = multiline::parse_multiline(&mut iter, &data)?;
                if !multiline.is_empty() {
                    map.insert(key, ParseResult::Vec(multiline));
                }
            }
        }
    }

    Ok(map)
}

/// Parses a packagebuild from the supplied input
/// # Arguments
/// * `input` - The packagebuild as an input implementing Read
pub fn parse<R: Read>(input: &mut R) -> Result<PackageBuild, std::io::Error> {
    // Get all the contents
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;

    // Get all the lines and parse the packagebuild
    let lines: Vec<&str> = contents.split('\n').collect();
    let entries = parse_pkgbuild(lines)?;

    // Parse `real_version` into a u32
    let real_version: u32 = match entries.get_str("real_version")?.parse() {
        Ok(v) => v,
        Err(e) => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Parsing real_version failed: {}", e),
            ))
        }
    };

    // Parse `strip` into a boolean
    let strip = entries.get_str_opt("strip")?.map(|s| s == "1");

    //Map the PackageBuild struct
    Ok(PackageBuild {
        name: entries.get_str("name")?,
        version: entries.get_str("version")?,
        real_version,

        maintainer: entries.get_str_opt("maintainer")?,
        maintainer_email: entries.get_str_opt("maintainer_email")?,
        description: entries.get_str_opt("description")?,
        provides: entries.get_vec_opt("provides")?,
        source: entries.get_str_opt("source")?,
        extra_sources: entries.get_vec_opt("extra_sources")?,
        extra_dependencies: entries.get_vec_opt("extra_dependencies")?,
        optional_dependencies: entries.get_vec_opt("optional_dependencies")?,
        build_dependencies: entries.get_vec_opt("build_dependencies")?,
        cross_dependencies: entries.get_vec_opt("cross_dependencies")?,
        preinstall: entries.get_str_opt("preinstall")?,
        postinstall: entries.get_str_opt("postinstall")?,
        strip,

        prepare: entries.get_vec_opt("prepare")?,
        build: entries.get_vec_opt("build")?,
        check: entries.get_vec_opt("check")?,
        package: entries.get_vec_opt("package")?,
    })
}
