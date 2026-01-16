use filtrum::common::WithFilterId;
use filtrum::equal_filter::EqualFilter;
use filtrum::errors::FilterParseError;
use filtrum::number_filter::{NumberFilter, NumberFilters};
use filtrum::query_filter::FromQueryFilter;
use filtrum::string_filter::{StringFilter, StringFilters};
use std::str::FromStr;

#[derive(Default, Debug)]
struct UserFilter {
    name: StringFilters,
    age: NumberFilters<i32>,
    active: EqualFilter<bool>,
}

impl WithFilterId for UserFilter {
    fn filter_id() -> Option<&'static str> {
        None
    }
}

impl FromStr for UserFilter {
    type Err = FilterParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(UserFilter {
            name: StringFilters::from_str("name", s)?,
            age: NumberFilters::from_str("age", s)?,
            active: EqualFilter::from_str("active", s)?,
        })
    }
}

#[test]
fn test_complex_query_parsing() {
    let query = "name[sw]=Ali&age[gte]=18&age[lt]=30&active=true&limit=10&skip=5&order_by[desc]=age";
    
    let filter = FromQueryFilter::<UserFilter>::from_str(query).expect("Failed to parse query");

    // Check inner filters
    let name_filters = filter.inner.name.0;
    assert!(name_filters.contains(&StringFilter::StartsWith("Ali".to_string())));

    let age_filters = filter.inner.age.0;
    assert!(age_filters.contains(&NumberFilter::Gte(18)));
    assert!(age_filters.contains(&NumberFilter::Lt(30)));

    let active_filter = filter.inner.active.into_inner();
    assert_eq!(active_filter, Some(true));

    // Check query params
    assert_eq!(filter.limit.map(|l| l.0), Some(10));
    assert_eq!(filter.skip.map(|s| s.0), Some(5));
    
    match filter.order_by {
        Some(filtrum::order_by::OrderBy::Desc(id)) => assert_eq!(id.id(), "age"),
        _ => panic!("Expected Desc order by age"),
    }
}

#[test]
fn test_empty_query() {
    let query = "";
    let filter = FromQueryFilter::<UserFilter>::from_str(query).expect("Failed to parse empty query");
    
    assert!(filter.inner.name.0.is_empty());
    assert!(filter.inner.age.0.is_empty());
    assert_eq!(filter.inner.active.into_inner(), None);
    assert!(filter.limit.is_none());
    assert!(filter.skip.is_none());
    assert!(filter.order_by.is_none());
}

#[test]
fn test_prefixed_filters() {
    // If we were using a prefix in WithFilterId, we would test it here.
    // For now, let's test that unrelated fields are ignored.
    let query = "name[eq]=Bob&other_field[eq]=ignored";
    let filter = FromQueryFilter::<UserFilter>::from_str(query).unwrap();

    let name_filters = filter.inner.name.0;
    assert_eq!(name_filters.len(), 1);
    assert!(name_filters.contains(&StringFilter::Eq("Bob".to_string())));
}
