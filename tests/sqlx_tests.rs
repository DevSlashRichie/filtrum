#![cfg(feature = "sqlx")]

use filtrum::{
    equal_filter::EqualFilter,
    number_filter::NumberFilters,
    query_filter::FromQueryFilter,
    sqlx::SqlxFilter,
    string_filter::StringFilters,
    WithFilterId,
};
use sqlx::{Sqlite, QueryBuilder};
use std::str::FromStr;

#[derive(Default)]
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
    type Err = filtrum::errors::FilterParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(UserFilter {
            name: StringFilters::from_str("name", s)?,
            age: NumberFilters::from_str("age", s)?,
            active: EqualFilter::from_str("active", s)?,
        })
    }
}

impl SqlxFilter<Sqlite> for UserFilter {
    fn apply<'a>(&self, qb: &mut QueryBuilder<'a, Sqlite>) {
        self.name.apply(qb);
        self.age.apply(qb);
        self.active.apply(qb);
    }
}

#[test]
fn test_sqlx_query_builder() {
    let query = "name[sw]=Ali&age[gte]=18&active=true&limit=10&skip=5&order_by[desc]=age";
    let filter = FromQueryFilter::<UserFilter>::from_str(query).expect("Failed to parse query");

    let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new("SELECT * FROM users WHERE 1=1");
    filter.apply(&mut qb);

    let sql = qb.sql();
    
    // Check SQL structure
    // Expected: SELECT * FROM users WHERE 1=1 AND name LIKE ? AND age >= ? AND active = ? ORDER BY age DESC LIMIT ? OFFSET ?
    
    // Note: Parameter placeholders in Sqlite are ? or $n depending on configuration, but usually ? in sqlx for sqlite? 
    // Wait, sqlx Sqlite uses ? or $N. Let's verify what `sql()` returns. 
    // It returns the query string.
    
    println!("Generated SQL: {}", sql);
    
    assert!(sql.contains("SELECT * FROM users WHERE 1=1"));
    assert!(sql.contains("AND name LIKE"));
    assert!(sql.contains("AND age >="));
    assert!(sql.contains("AND active ="));
    assert!(sql.contains("ORDER BY age DESC"));
    assert!(sql.contains("LIMIT"));
    assert!(sql.contains("OFFSET"));
}
