use crate::{
    common::{from_str, FromStrFilter},
    errors::FilterParseError,
    filter_id::FilterId,
};

/// Represents sorting instructions.
///
/// Parses `order_by[asc]=field` or `order_by[desc]=field`.
///
/// # Example
///
/// ```rust
/// use filtrum::order_by::OrderBy;
///
/// let query = "order_by[desc]=created_at";
/// let order = OrderBy::from_str(query).unwrap().unwrap();
///
/// match order {
///     OrderBy::Desc(id) => assert_eq!(id.id(), "created_at"),
///     _ => panic!("Expected Desc"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderBy {
    /// Ascending order.
    Asc(FilterId),
    /// Descending order.
    Desc(FilterId),
}

impl FromStrFilter<String> for OrderBy {
    fn from_str(id: &str, value: String) -> Result<Self, FilterParseError> {
        match id {
            "asc" => Ok(OrderBy::Asc(value.into())),
            "desc" => Ok(OrderBy::Desc(value.into())),
            _ => Err(FilterParseError::UnknownFilter)?,
        }
    }
}

impl OrderBy {
    pub fn from_str(value: &str) -> Result<Option<Self>, FilterParseError> {
        let u = from_str("order_by", value)?.first().cloned();

        Ok(u)
    }

    pub fn from_str_prefix(prefix: &str, value: &str) -> Result<Option<Self>, FilterParseError> {
        let data = Self::from_str(value)?.map(|x| -> OrderBy {
            match x {
                OrderBy::Asc(u) => match u {
                    FilterId::Alone(value) => {
                        OrderBy::Asc(FilterId::WithPrefix(prefix.to_string(), value))
                    }
                    _ => unreachable!(),
                },
                OrderBy::Desc(u) => match u {
                    FilterId::Alone(value) => {
                        OrderBy::Desc(FilterId::WithPrefix(prefix.to_string(), value))
                    }
                    _ => unreachable!(),
                },
            }
        });
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_by() {
        let qs = "order_by[asc]=name";
        let ob = OrderBy::from_str(qs).unwrap().unwrap();
        match ob {
            OrderBy::Asc(id) => assert_eq!(id.id(), "name"),
            _ => panic!("Expected Asc"),
        }

        let qs = "order_by[desc]=age";
        let ob = OrderBy::from_str(qs).unwrap().unwrap();
        match ob {
            OrderBy::Desc(id) => assert_eq!(id.id(), "age"),
            _ => panic!("Expected Desc"),
        }
    }

    #[test]
    fn test_order_by_prefix() {
        let qs = "order_by[asc]=name";
        let ob = OrderBy::from_str_prefix("user", qs).unwrap().unwrap();
        match ob {
            OrderBy::Asc(id) => {
                assert_eq!(id.id(), "name");
                assert_eq!(id.prefix(), Some("user"));
            }
            _ => panic!("Expected Asc"),
        }
    }
}