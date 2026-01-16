use std::str::FromStr;

use crate::{
    common::WithFilterId, errors::FilterParseError, limit::Limit, order_by::OrderBy, skip::Skip,
};

/// A container for parsing and holding query filters, including pagination and sorting.
///
/// This struct is typically used to parse a query string into a structured representation
/// containing domain-specific filters (`inner`), as well as standard `order_by`, `limit`, and `skip` parameters.
///
/// # Type Parameters
///
/// * `T`: The type representing the domain-specific filters (e.g., a struct with fields for `name`, `age`, etc.).
///        Must implement `FromStr`, `WithFilterId`, and `Default`.
///
/// # Example
///
/// ```rust
/// use filtrum::query_filter::FromQueryFilter;
/// use filtrum::equal_filter::EqualFilter;
/// use filtrum::WithFilterId;
/// use std::str::FromStr;
/// use filtrum::errors::FilterParseError;
///
/// #[derive(Default)]
/// struct UserFilter {
///     age: EqualFilter<i32>,
/// }
///
/// impl WithFilterId for UserFilter {
///     fn filter_id() -> Option<&'static str> { None }
/// }
///
/// impl FromStr for UserFilter {
///     type Err = FilterParseError;
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///         Ok(UserFilter {
///             age: EqualFilter::from_str("age", s)?,
///         })
///     }
/// }
///
/// let query = "age=30&limit=10&skip=5";
/// let filter = FromQueryFilter::<UserFilter>::from_str(query).unwrap();
///
/// assert_eq!(filter.inner.age.into_inner(), Some(30));
/// assert_eq!(filter.limit.map(|l| l.0), Some(10));
/// assert_eq!(filter.skip.map(|s| s.0), Some(5));
/// ```
#[derive(Debug, Default, Clone)]
pub struct FromQueryFilter<T: FromStr + WithFilterId + Default> {
    /// The domain-specific filters.
    pub inner: T,
    /// Sorting instruction, if present.
    pub order_by: Option<OrderBy>,
    /// Limit for pagination, if present.
    pub limit: Option<Limit>,
    /// Skip (offset) for pagination, if present.
    pub skip: Option<Skip>,
}

impl<T: Default> FromQueryFilter<T>
where
    T: FromStr<Err = FilterParseError> + WithFilterId,
{
    /// Parses a query string into a `FromQueryFilter` instance.
    ///
    /// This method extracts standard parameters (`order_by`, `limit`, `skip`) and delegates
    /// the parsing of the inner filter type `T` to its `FromStr` implementation.
    ///
    /// # Arguments
    ///
    /// * `value`: The query string to parse (e.g., "key=value&limit=10").
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
    /// Creates an empty `FromQueryFilter` with default values.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Creates a `FromQueryFilter` from an existing inner filter instance, with no other options set.
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