use std::sync::OnceLock;

use regex::Regex;

static QUERY_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn query_regex() -> &'static Regex {
    QUERY_REGEX.get_or_init(|| Regex::new(r"(\w+)(\[([a-z]+)])?(\[(\d+)])?").unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_matching() {
        let re = query_regex();

        // case: age[eq]
        let caps = re.captures("age[eq]").unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "age");
        assert_eq!(caps.get(3).unwrap().as_str(), "eq");

        // case: age
        let caps = re.captures("age").unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "age");
        assert!(caps.get(3).is_none());

        // case: field[op][index]
        // age[eq][10]
        let caps = re.captures("age[eq][10]").unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "age");
        assert_eq!(caps.get(3).unwrap().as_str(), "eq");
        assert_eq!(caps.get(5).unwrap().as_str(), "10");
    }
}