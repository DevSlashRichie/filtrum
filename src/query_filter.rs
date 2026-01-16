use std::str::FromStr;

use crate::{
    common::WithFilterId, errors::FilterParseError, limit::Limit, order_by::OrderBy, skip::Skip,
};

#[derive(Debug, Default, Clone)]
pub struct FromQueryFilter<T: FromStr + WithFilterId + Default> {
    pub inner: T,
    pub order_by: Option<OrderBy>,
    pub limit: Option<Limit>,
    pub skip: Option<Skip>,
}

impl<T: Default> FromQueryFilter<T>
where
    T: FromStr<Err = FilterParseError> + WithFilterId,
{
    pub fn from_str(value: &str) -> Result<Self, FilterParseError> {
        let order_by = if let Some(prefix) = T::filter_id() {
            OrderBy::from_str_prefix(prefix, value)?
        } else {
            OrderBy::from_str(value)?
        };

        let limit = Limit::from_str(value)?;

        let skip = Skip::from_str(value)?;

        let inner = T::from_str(value)?;

        Ok(Self {
            order_by,
            limit,
            inner,
            skip,
        })
    }
}

impl<T> FromQueryFilter<T>
where
    T: FromStr + WithFilterId + Default,
{
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn from_inner(inner: T) -> Self {
        Self {
            inner,
            ..Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::equal_filter::EqualFilter;

    #[derive(Default)]
    struct MockQuery {
        age: EqualFilter<i32>,
    }

    impl crate::common::WithFilterId for MockQuery {
        fn filter_id() -> Option<&'static str> {
            None
        }
    }

    impl FromStr for MockQuery {
        type Err = FilterParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(MockQuery {
                age: EqualFilter::from_str("age", s)?,
            })
        }
    }

    #[test]
    fn test_from_query_filter() {
        let qs = "age=20&limit=10&order_by[asc]=age";
        let q: FromQueryFilter<MockQuery> = FromQueryFilter::from_str(qs).unwrap();

        assert_eq!(q.inner.age.into_inner(), Some(20));
        assert_eq!(q.limit.unwrap().0, 10);
        match q.order_by.unwrap() {
            OrderBy::Asc(id) => assert_eq!(id.id(), "age"),
            _ => panic!("Expected Asc"),
        }
    }
}