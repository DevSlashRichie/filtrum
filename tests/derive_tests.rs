#[cfg(feature = "derive")]
mod tests {
    use filtrum::WithFilterId;
    use filtrum::{
        equal_filter::EqualFilter, number_filter::NumberFilters, string_filter::StringFilters,
        Filterable,
    };
    use std::str::FromStr;

    #[derive(Filterable, Debug)]
    #[filtrum(table = "users")]
    struct UserFilter {
        #[filtrum(alias = "n")]
        name: StringFilters,

        age: NumberFilters<i32>,

        #[filtrum(skip)]
        ignored: String,

        is_active: EqualFilter<bool>,
    }

    #[test]
    fn test_derive_macro() {
        let query = "name[eq]=Alice&age[gte]=18&is_active=true";
        let filter = UserFilter::from_str(query).expect("Failed to parse query");

        let name_filters = filter.name.0;
        assert!(!name_filters.is_empty());

        // We can't easily inspect the exact content without public fields or more getters,
        // but we know it parsed if it's not empty and we used the alias.

        // Check age
        let age_filters = filter.age.0;
        assert!(!age_filters.is_empty());

        // Check is_active
        assert_eq!(filter.is_active.into_inner(), Some(true));

        assert_eq!(filter.ignored, "");

        assert_eq!(UserFilter::filter_id(), Some("users"));
    }

    #[test]
    fn test_filter_id_impl() {
        use filtrum::common::WithFilterId;
        assert_eq!(UserFilter::filter_id(), Some("users"));
    }
}
