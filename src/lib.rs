pub mod mount;
pub mod parser;

use serde::{de::*, *};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageBuild {
    pub name: String,
    pub version: String,
    #[serde(deserialize_with = "crate::deserialize_number_from_string")]
    pub real_version: u32,

    pub maintainer: Option<String>,
    pub maintainer_email: Option<String>,
    pub description: Option<String>,
    pub provides: Option<Vec<String>>,
    pub source: Option<String>,
    pub extra_sources: Option<Vec<String>>,
    pub extra_dependencies: Option<Vec<String>>,
    pub optional_dependencies: Option<Vec<String>>,
    pub build_dependencies: Option<Vec<String>>,
    pub cross_dependencies: Option<Vec<String>>,
    pub preinstall: Option<String>,
    pub postinstall: Option<String>,
    pub strip: Option<bool>,

    pub prepare: Option<Vec<String>>,
    pub build: Option<Vec<String>>,
    pub check: Option<Vec<String>>,
    pub package: Option<Vec<String>>,
}

impl PackageBuild {
    pub fn new(name: &str, version: &str, real_version: u32) -> PackageBuild {
        PackageBuild {
            name: name.to_owned(),
            version: version.to_owned(),
            real_version,

            maintainer: None,
            maintainer_email: None,
            description: None,
            provides: None,
            source: None,
            extra_sources: None,
            extra_dependencies: None,
            optional_dependencies: None,
            build_dependencies: None,
            cross_dependencies: None,
            preinstall: None,
            postinstall: None,
            strip: None,

            prepare: None,
            build: None,
            check: None,
            package: None,
        }
    }
}

/// Deserializes a integer from a string
pub fn deserialize_number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Number(T),
    }

    match StringOrInt::<T>::deserialize(deserializer)? {
        StringOrInt::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
        StringOrInt::Number(i) => Ok(i),
    }
}
