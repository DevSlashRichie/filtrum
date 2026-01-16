use std::str::FromStr;

use serde::{
    de::{self, Visitor},
    Deserialize,
};

use crate::{
    common::{from_str, FromStrFilter},
    errors::FilterParseError,
    filter_id::FilterId,
};

/// Represents numerical comparison operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumberFilter<T> {
    /// Equal (`=`). Query param: `field[eq]=10` or `field=10` (inferred).
    Eq(T),
    /// Not equal (`<>` or `!=`). Query param: `field[ne]=10`.
    Ne(T),
    /// Greater than (`>`). Query param: `field[gt]=10`.
    Gt(T),
    /// Less than (`<`). Query param: `field[lt]=10`.
    Lt(T),
    /// Greater than or equal (`>=`). Query param: `field[gte]=10`.
    Gte(T),
    /// Less than or equal (`<=`). Query param: `field[lte]=10`.
    Lte(T),
}

impl<T> FromStrFilter<T> for NumberFilter<T> {
    fn from_str(id: &str, value: T) -> Result<Self, FilterParseError> {
        let f = match id {
            "eq" => NumberFilter::Eq(value),
            "ne" => NumberFilter::Ne(value),
            "gt" => NumberFilter::Gt(value),
            "lt" => NumberFilter::Lt(value),
            "gte" => NumberFilter::Gte(value),
            "lte" => NumberFilter::Lte(value),
            _ => Err(FilterParseError::UnknownFilter)?,
        };

        Ok(f)
    }
}

/// A collection of number filters applied to a specific field.
///
/// # Example
///
/// ```rust
/// use filtrum::number_filter::{NumberFilters, NumberFilter};
///
/// let query = "age[gte]=18&age[lt]=65";
/// let filters = NumberFilters::<i32>::from_str("age", query).unwrap();
///
/// assert!(filters.0.contains(&NumberFilter::Gte(18)));
/// assert!(filters.0.contains(&NumberFilter::Lt(65)));
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct NumberFilters<T>(pub Vec<NumberFilter<T>>, pub Option<FilterId>);

impl<T: FromStr> NumberFilters<T> {
    /// Parses number filters from a query string for a specific search ID.
    pub fn from_str(search_id: &str, value: &str) -> Result<Self, FilterParseError> {
        Self::from_id_value(search_id.to_string().into(), value)
    }

    /// Parses number filters from a query string for a specific `FilterId`.
    pub fn from_id_value(search_id: FilterId, value: &str) -> Result<Self, FilterParseError> {
        from_str(search_id.id(), value).map(|x| Self(x, Some(search_id)))
    }
}

impl<'de, T, E> Deserialize<'de> for NumberFilter<T>
where
    T: Deserialize<'de> + FromStr<Err = E>,
    E: std::fmt::Debug,
{
    // lte=10 or eq=10 or gte=10
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct NumberFilterVisitor<T>(std::marker::PhantomData<T>);

        impl<'de, T, H> Visitor<'de> for NumberFilterVisitor<T>
        where
            T: Deserialize<'de> + FromStr<Err = H>,
            H: std::fmt::Debug,
        {
            type Value = NumberFilter<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a number filter")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let parts = v.split('=').collect::<Vec<_>>();

                if parts.len() != 2 {
                    // we infer is equals
                    return Ok(NumberFilter::Eq(v.parse().map_err(|_|
                        de::Error::invalid_value(
                            de::Unexpected::Str(v),
                            &"a number filter value in dual format",
                        )
                    )?));
                }

                let (key, value) = (parts[0], parts[1]);

                let value = value.parse().map_err(|err| {
                    let error_msg = format!("a number in filter value: {:?}", err);
                    de::Error::invalid_value(de::Unexpected::Str(value), &error_msg.as_str())
                })?;

                match key {
                    "eq" => Ok(NumberFilter::Eq(value)),
                    "ne" => Ok(NumberFilter::Ne(value)),
                    "gt" => Ok(NumberFilter::Gt(value)),
                    "lt" => Ok(NumberFilter::Lt(value)),
                    "gte" => Ok(NumberFilter::Gte(value)),
                    "lte" => Ok(NumberFilter::Lte(value)),
                    _ => Err(de::Error::custom("unknown number filter")),
                }
            }
        }

        deserializer.deserialize_str(NumberFilterVisitor(std::marker::PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_filter_parsing() {
        let qs = "age[gte]=18&age[lt]=100";
        let f = NumberFilters::<i32>::from_str("age", qs).unwrap();
        let filters = f.0;
        assert_eq!(filters.len(), 2);
        assert!(filters.contains(&NumberFilter::Gte(18)));
        assert!(filters.contains(&NumberFilter::Lt(100)));
    }

    #[test]
    fn test_deserialization() {
        // "gte=10"
        let f: NumberFilter<i32> = serde_json::from_str("\"gte=10\"").unwrap();
        assert_eq!(f, NumberFilter::Gte(10));

        // "10" -> Eq(10)
        let f: NumberFilter<i32> = serde_json::from_str("\"10\"").unwrap();
        assert_eq!(f, NumberFilter::Eq(10));
    }
}