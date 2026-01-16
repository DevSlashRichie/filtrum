use std::str::FromStr;

use crate::{
    common::{from_str, FromStrFilter},
    errors::FilterParseError,
    filter_id::FilterId,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EqualFilter<T>(pub Option<T>, pub Option<FilterId>);

impl<T> EqualFilter<T> {
    pub fn into_inner(self) -> Option<T> {
        self.0
    }

    pub fn as_ref(&self) -> Option<&T> {
        self.0.as_ref()
    }
}

impl<T> FromStrFilter<T> for EqualFilter<T> {
    fn from_str(_filter_key: &str, value: T) -> Result<Self, FilterParseError> {
        // we don't need to the filter id here so we ignore it.
        Ok(Self(Some(value), Some("".to_string().into())))
    }
}

impl<T> EqualFilter<T>
where
    T: FromStr + Clone,
{
    pub fn from_str(search_id: &str, value: &str) -> Result<Self, FilterParseError> {
        Self::from_id_value(search_id.to_string().into(), value)
    }

    pub fn from_id_value(search_id: FilterId, value: &str) -> Result<Self, FilterParseError> {
        // we use the same algorithm as others, but we ignore the filter
        let u = from_str::<T, EqualFilter<T>>(search_id.id(), value)?
            .first()
            .cloned();

        match u {
            Some(u) => Ok(Self(u.0, Some(search_id))),
            None => Ok(Self(None, Some(search_id))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_filter_from_str() {
        let qs = "age=20";
        let f = EqualFilter::<i32>::from_str("age", qs).unwrap();
        assert_eq!(f.into_inner(), Some(20));

        let qs = "height=20";
        let f = EqualFilter::<i32>::from_str("age", qs).unwrap();
        assert_eq!(f.into_inner(), None);
    }
}