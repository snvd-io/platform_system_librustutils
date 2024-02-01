//! Parsers and formatters.
//!
//! These functions should only be used in the system properties generated code.

use std::str::FromStr;
use std::string::ToString;

type Result<T> = std::result::Result<T, String>;

// Parsers.

#[allow(missing_docs)]
pub fn parse<T: FromStr>(s: &str) -> Result<T> {
    s.parse::<T>()
        .map_err(|_| format!("Can't convert '{}' to '{}'.", s, std::any::type_name::<T>()))
}

#[allow(missing_docs)]
pub fn parse_bool(s: &str) -> Result<bool> {
    match s {
        "1" | "true" => Ok(true),
        "0" | "false" => Ok(false),
        _ => Err(format!("Can't convert '{}' to 'bool'.", s)),
    }
}

fn parse_list_with<T, F>(s: &str, f: F) -> Result<Vec<T>>
where
    F: Fn(&str) -> Result<T>,
{
    let mut result = Vec::new();
    if s.is_empty() {
        return Ok(result);
    }

    let mut chars = s.chars();
    let mut current = chars.next();
    while current.is_some() {
        // Extract token.
        let mut token = String::with_capacity(s.len());
        while let Some(value) = current {
            if value == ',' {
                break;
            }
            if value == '\\' {
                current = chars.next()
            }
            if let Some(value) = current {
                token.push(value);
            }
            current = chars.next();
        }
        // Parse token.
        result.push(f(token.as_str())?);
        current = chars.next()
    }

    Ok(result)
}

#[allow(missing_docs)]
pub fn parse_list<T: FromStr>(s: &str) -> Result<Vec<T>> {
    parse_list_with(s, parse)
}

#[allow(missing_docs)]
pub fn parse_bool_list(s: &str) -> Result<Vec<bool>> {
    parse_list_with(s, parse_bool)
}

// Formatters.

#[allow(missing_docs)]
pub fn format<T: ToString>(v: &T) -> String {
    v.to_string()
}

#[allow(missing_docs)]
pub fn format_bool(v: &bool) -> String {
    if *v {
        return "true".into();
    }
    "false".into()
}

#[allow(missing_docs)]
pub fn format_bool_as_int(v: &bool) -> String {
    if *v {
        return "1".into();
    }
    "0".into()
}

fn format_list_with<T, F>(v: &[T], f: F) -> String
where
    F: Fn(&T) -> String,
{
    let mut result = String::new();
    for item in v {
        let formatted = f(item);
        result.push_str(formatted.as_str());
        result.push(',');
    }
    result.pop();
    result
}

#[allow(missing_docs)]
pub fn format_list<T: ToString>(v: &[T]) -> String {
    format_list_with(v, format)
}

#[allow(missing_docs)]
pub fn format_bool_list(v: &[bool]) -> String {
    format_list_with(v, format_bool)
}

#[allow(missing_docs)]
pub fn format_bool_list_as_int(v: &[bool]) -> String {
    format_list_with(v, format_bool_as_int)
}
