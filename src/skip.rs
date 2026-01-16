use crate::{
    common::{from_str, FromStrFilter},
    errors::FilterParseError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Skip(pub u64);

impl FromStrFilter<u64> for Skip {
    fn from_str(_id: &str, value: u64) -> Result<Self, FilterParseError> {
        Ok(Skip(value))
    }
}

impl Skip {
    pub fn from_str(value: &str) -> Result<Option<Self>, FilterParseError> {
        let u = from_str("skip", value)?.first().cloned();

        Ok(u)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_from_str() {
        let qs = "skip=10";
        let s = Skip::from_str(qs).unwrap().unwrap();
        assert_eq!(s.0, 10);

        let qs = "other=10";
        let s = Skip::from_str(qs).unwrap();
        assert!(s.is_none());

        // invalid
        let qs = "skip=abc";
        let s = Skip::from_str(qs);
        assert!(s.is_err());
    }
}