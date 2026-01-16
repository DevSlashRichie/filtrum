use crate::{
    common::{from_str, FromStrFilter},
    errors::FilterParseError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limit(pub u64);

impl FromStrFilter<u64> for Limit {
    fn from_str(_id: &str, value: u64) -> Result<Self, FilterParseError> {
        Ok(Limit(value))
    }
}

impl Limit {
    pub fn from_str(value: &str) -> Result<Option<Self>, FilterParseError> {
        let u = from_str("limit", value)?.first().cloned();

        Ok(u)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit_from_str() {
        let qs = "limit=50";
        let l = Limit::from_str(qs).unwrap().unwrap();
        assert_eq!(l.0, 50);

        let qs = "other=50";
        let l = Limit::from_str(qs).unwrap();
        assert!(l.is_none());
    }
}