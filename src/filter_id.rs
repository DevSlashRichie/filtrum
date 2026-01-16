#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterId {
    Alone(String),
    WithPrefix(String, String),
    WithPrefixAndAlias(String, String, String),
}

impl FilterId {
    pub fn id(&self) -> &str {
        match self {
            FilterId::Alone(id) => id,
            FilterId::WithPrefix(_, id) => id,
            FilterId::WithPrefixAndAlias(_, id, _) => id,
        }
    }

    pub fn prefix(&self) -> Option<&str> {
        match self {
            FilterId::Alone(_) => None,
            FilterId::WithPrefix(prefix, _) => Some(prefix),
            FilterId::WithPrefixAndAlias(prefix, _, _) => Some(prefix),
        }
    }

    pub fn key(&self) -> &str {
        match self {
            FilterId::Alone(id) => id,
            FilterId::WithPrefix(_, id) => id,
            FilterId::WithPrefixAndAlias(_, _, alias) => alias,
        }
    }
}

impl From<String> for FilterId {
    fn from(value: String) -> Self {
        Self::Alone(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_id_methods() {
        let f1 = FilterId::Alone("age".to_string());
        assert_eq!(f1.id(), "age");
        assert_eq!(f1.prefix(), None);

        let f2 = FilterId::WithPrefix("user".to_string(), "age".to_string());
        assert_eq!(f2.id(), "age");
        assert_eq!(f2.prefix(), Some("user"));

        let f3 = FilterId::WithPrefixAndAlias(
            "user".to_string(),
            "age".to_string(),
            "a".to_string(),
        );
        assert_eq!(f3.id(), "age");
        assert_eq!(f3.prefix(), Some("user"));
    }

    #[test]
    fn test_from_string() {
        let f: FilterId = "age".to_string().into();
        assert_eq!(f, FilterId::Alone("age".to_string()));
    }
}