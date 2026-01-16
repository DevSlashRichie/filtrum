use std::{fmt::Display, str::FromStr};

use serde::{de, Deserialize};

use crate::{
    common::{from_str, FromStrFilter},
    errors::FilterParseError,
    filter_id::FilterId,
};

/// Represents various string comparison operations.
///
/// This enum corresponds to common string filtering operations found in databases and APIs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StringFilter<T = String> {
    /// Exact match (`=`). Query param: `field[eq]=value` or `field=value` (inferred).
    Eq(T),
    /// Not equal (`<>` or `!=`). Query param: `field[ne]=value`.
    Ne(T),
    /// SQL LIKE match. Query param: `field[like]=value` or `field[l]=value`.
    Like(T),
    /// SQL NOT LIKE match. Query param: `field[not_like]=value` or `field[nl]=value`.
    NotLike(T),
    /// Starts with match (`LIKE 'value%'`). Query param: `field[starts_with]=value` or `field[sw]=value`.
    StartsWith(T),
    /// Ends with match (`LIKE '%value'`). Query param: `field[ends_with]=value` or `field[ew]=value`.
    EndsWith(T),
    /// Contains match (`LIKE '%value%'`). Query param: `field[contains]=value` or `field[c]=value`.
    Contains(T),
}

impl<T> FromStrFilter<T> for StringFilter<T>
where
    T: FromStr,
{
    fn from_str(id: &str, value: T) -> Result<Self, FilterParseError> {
        match id {
            "eq" => Ok(StringFilter::Eq(value)),
            "ne" => Ok(StringFilter::Ne(value)),

            "like" | "l" => Ok(StringFilter::Like(value)),

            "not_like" | "nl" => Ok(StringFilter::NotLike(value)),

            "starts_with" | "sw" => Ok(StringFilter::StartsWith(value)),

            "ends_with" | "ew" => Ok(StringFilter::EndsWith(value)),

            "contains" | "c" => Ok(StringFilter::Contains(value)),

            _ => Err(FilterParseError::UnknownFilter),
        }
    }
}

/// A collection of string filters applied to a specific field.
///
/// This struct holds a list of `StringFilter`s that should be applied to the field identified by `FilterId`.
///
/// # Example
///
/// ```rust
/// use filtrum::string_filter::{StringFilters, StringFilter};
/// use std::str::FromStr; // Import the trait!
///
/// let query = "name[sw]=Al&name[ne]=Alice";
/// let filters = StringFilters::<String>::from_str("name", query).unwrap();
///
/// assert!(filters.0.contains(&StringFilter::StartsWith("Al".to_string())));
/// assert!(filters.0.contains(&StringFilter::Ne("Alice".to_string())));
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct StringFilters<T = String>(pub Vec<StringFilter<T>>, pub Option<FilterId>)
where
    T: FromStr + Display;

impl<T> StringFilters<T>
where
    T: FromStr + Display,
{
    /// Parses string filters from a query string for a specific search ID.
    pub fn from_str(search_id: &str, value: &str) -> Result<Self, FilterParseError> {
        Self::from_id_value(search_id.to_string().into(), value)
    }

    /// Parses string filters from a query string for a specific `FilterId`.
    pub fn from_id_value(search_id: FilterId, value: &str) -> Result<Self, FilterParseError> {
        from_str(search_id.id(), value).map(|x| Self(x, Some(search_id)))
    }
}

impl<'de> Deserialize<'de> for StringFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StringFilterVisitor;

        impl<'de> de::Visitor<'de> for StringFilterVisitor {
            type Value = StringFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string filter")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let parts = v.split('=').collect::<Vec<_>>();

                if parts.len() != 2 {
                    // we infer is equals
                    return Ok(StringFilter::Eq(v.to_string()));
                }

                let (key, value) = (parts[0], parts[1]);

                match key {
                    "eq" => Ok(StringFilter::Eq(value.to_string())),
                    "ne" => Ok(StringFilter::Ne(value.to_string())),

                    "like" | "l" => Ok(StringFilter::Like(value.to_string())),

                    "not_like" | "nl" => Ok(StringFilter::NotLike(value.to_string())),

                    "starts_with" | "sw" => Ok(StringFilter::StartsWith(value.to_string())),

                    "ends_with" | "ew" => Ok(StringFilter::EndsWith(value.to_string())),

                    "contains" | "c" => Ok(StringFilter::Contains(value.to_string())),

                    _ => Err(de::Error::custom("unknown string filter")),
                }
            }
        }

        deserializer.deserialize_str(StringFilterVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_filter_parsing() {
        let qs = "name[like]=john&name[ne]=doe";
        let f = StringFilters::<String>::from_str("name", qs).unwrap();
        let filters = f.0;
        assert_eq!(filters.len(), 2);
        assert!(filters.contains(&StringFilter::Like("john".to_string())));
        assert!(filters.contains(&StringFilter::Ne("doe".to_string())));
    }

    #[test]
    fn test_string_deserialization() {
        let f: StringFilter = serde_json::from_str("\"like=john\"").unwrap();
        assert_eq!(f, StringFilter::Like("john".to_string()));

        let f: StringFilter = serde_json::from_str("\"john\"").unwrap();
        assert_eq!(f, StringFilter::Eq("john".to_string()));
    }
}