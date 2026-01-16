use std::str::FromStr;

use crate::{errors::FilterParseError, regex::query_regex};

pub trait FromStrFilter<T>: Sized {
    fn from_str(filter_key: &str, value: T) -> Result<Self, FilterParseError>;
}

pub fn from_str<V, T>(search_id: &str, value: &str) -> Result<Vec<T>, FilterParseError>
where
    T: FromStrFilter<V>,
    V: FromStr,
{
    if value.is_empty() {
        return Ok(Vec::new());
    }

    // age[lte]=10&age[gte]=20&age[eq]=30
    let mut filters = Vec::new();

    for part in value.split('&') {
        let (id_and_filter, value) = part
            .split_once('=')
            .ok_or(FilterParseError::FilterStructure)?;

        let rg = query_regex()
            .captures(id_and_filter)
            .ok_or(FilterParseError::FilterStructure)?;

        let id = rg.get(1).ok_or(FilterParseError::FilterStructure)?.as_str();

        let filter = rg.get(3).map_or("eq", |x| x.as_str());

        if id != search_id {
            continue;
        }

        let value = value.parse().map_err(|_| FilterParseError::Value)?;

        let filter = T::from_str(filter, value)?;
        filters.push(filter);
    }

    Ok(filters)
}

pub trait WithFilterId {
    fn filter_id() -> Option<&'static str>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::FilterParseError;

    struct MockFilter(String, i32);

    impl FromStrFilter<i32> for MockFilter {
        fn from_str(filter_key: &str, value: i32) -> Result<Self, FilterParseError> {
            Ok(MockFilter(filter_key.to_string(), value))
        }
    }

    #[test]
    fn test_from_str_empty() {
        let res: Vec<MockFilter> = from_str("age", "").unwrap();
        assert!(res.is_empty());
    }

    #[test]
    fn test_from_str_basic() {
        let qs = "age[eq]=10&age[lt]=20";
        let res: Vec<MockFilter> = from_str("age", qs).unwrap();
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].0, "eq");
        assert_eq!(res[0].1, 10);
        assert_eq!(res[1].0, "lt");
        assert_eq!(res[1].1, 20);
    }

    #[test]
    fn test_from_str_ignore_other_ids() {
        let qs = "age[eq]=10&height[eq]=20";
        let res: Vec<MockFilter> = from_str("age", qs).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].1, 10);
    }

    #[test]
    fn test_from_str_default_op() {
        let qs = "age=10";
        let res: Vec<MockFilter> = from_str("age", qs).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].0, "eq");
        assert_eq!(res[0].1, 10);
    }

    #[test]
    fn test_invalid_structure() {
        let qs = "age";
        let res: Result<Vec<MockFilter>, _> = from_str("age", qs);
        assert!(matches!(res, Err(FilterParseError::FilterStructure)));
    }

    #[test]
    fn test_invalid_value() {
        let qs = "age[eq]=notanumber";
        let res: Result<Vec<MockFilter>, _> = from_str("age", qs);
        assert!(matches!(res, Err(FilterParseError::Value)));
    }
}