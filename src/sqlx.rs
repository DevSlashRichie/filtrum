use sqlx::{Database, Encode, QueryBuilder, Type};
use std::fmt::Display;
use std::str::FromStr;

use crate::{
    equal_filter::EqualFilter,
    limit::Limit,
    number_filter::{NumberFilter, NumberFilters},
    order_by::OrderBy,
    query_filter::FromQueryFilter,
    skip::Skip,
    string_filter::{StringFilter, StringFilters},
};

/// A trait for applying filters to a `sqlx::QueryBuilder`.
///
/// This trait allows converting structured filters into SQL clauses appended to a query builder.
///
/// # Example
///
/// ```rust,ignore
/// use filtrum::sqlx::SqlxFilter;
/// use sqlx::{Postgres, QueryBuilder};
///
/// // Assume `filter` is a `FromQueryFilter<UserFilter>` populated from a request
/// let mut qb = QueryBuilder::<Postgres>::new("SELECT * FROM users WHERE 1=1");
///
/// filter.apply(&mut qb);
///
/// let query = qb.build();
/// ```
pub trait SqlxFilter<DB: Database> {
    /// Appends the filter conditions to the given query builder.
    fn apply<'a>(&self, query_builder: &mut QueryBuilder<'a, DB>);
}

impl<DB, T> SqlxFilter<DB> for StringFilters<T>
where
    DB: Database,
    T: Clone + Display + Send + Sync + 'static + FromStr,
    String: Type<DB> + for<'q> Encode<'q, DB>,
    T: Type<DB> + for<'q> Encode<'q, DB>,
{
    fn apply<'a>(&self, qb: &mut QueryBuilder<'a, DB>) {
        if let Some(col_id) = &self.1 {
            let col_name = col_id.key();
            for filter in &self.0 {
                qb.push(" AND ");
                qb.push(col_name);
                match filter {
                    StringFilter::Eq(v) => {
                        qb.push(" = ");
                        qb.push_bind(v.clone());
                    }
                    StringFilter::Ne(v) => {
                        qb.push(" <> ");
                        qb.push_bind(v.clone());
                    }
                    StringFilter::Like(v) => {
                        qb.push(" LIKE ");
                        qb.push_bind(format!("{}", v));
                    }
                    StringFilter::NotLike(v) => {
                        qb.push(" NOT LIKE ");
                        qb.push_bind(format!("{}", v));
                    }
                    StringFilter::StartsWith(v) => {
                        qb.push(" LIKE ");
                        qb.push_bind(format!("{}%", v));
                    }
                    StringFilter::EndsWith(v) => {
                        qb.push(" LIKE ");
                        qb.push_bind(format!("%{}", v));
                    }
                    StringFilter::Contains(v) => {
                        qb.push(" LIKE ");
                        qb.push_bind(format!("%{}%", v));
                    }
                }
            }
        }
    }
}

impl<DB, T> SqlxFilter<DB> for NumberFilters<T>
where
    DB: Database,
    T: Clone + Send + Sync + 'static,
    T: Type<DB> + for<'q> Encode<'q, DB>,
{
    fn apply<'a>(&self, qb: &mut QueryBuilder<'a, DB>) {
        if let Some(col_id) = &self.1 {
            let col_name = col_id.key();
            for filter in &self.0 {
                qb.push(" AND ");
                qb.push(col_name);
                match filter {
                    NumberFilter::Eq(v) => {
                        qb.push(" = ");
                        qb.push_bind(v.clone());
                    }
                    NumberFilter::Ne(v) => {
                        qb.push(" <> ");
                        qb.push_bind(v.clone());
                    }
                    NumberFilter::Gt(v) => {
                        qb.push(" > ");
                        qb.push_bind(v.clone());
                    }
                    NumberFilter::Lt(v) => {
                        qb.push(" < ");
                        qb.push_bind(v.clone());
                    }
                    NumberFilter::Gte(v) => {
                        qb.push(" >= ");
                        qb.push_bind(v.clone());
                    }
                    NumberFilter::Lte(v) => {
                        qb.push(" <= ");
                        qb.push_bind(v.clone());
                    }
                }
            }
        }
    }
}

impl<DB, T> SqlxFilter<DB> for EqualFilter<T>
where
    DB: Database,
    T: Clone + Send + Sync + 'static,
    T: Type<DB> + for<'q> Encode<'q, DB>,
{
    fn apply<'a>(&self, qb: &mut QueryBuilder<'a, DB>) {
        if let Some(val) = &self.0 {
            if let Some(col_id) = &self.1 {
                qb.push(" AND ");
                qb.push(col_id.key());
                qb.push(" = ");
                qb.push_bind(val.clone());
            }
        }
    }
}

impl<DB> SqlxFilter<DB> for Limit
where
    DB: Database,
    i64: Type<DB> + for<'q> Encode<'q, DB>,
{
    fn apply<'a>(&self, qb: &mut QueryBuilder<'a, DB>) {
        qb.push(" LIMIT ");
        qb.push_bind(self.0 as i64);
    }
}

impl<DB> SqlxFilter<DB> for Skip
where
    DB: Database,
    i64: Type<DB> + for<'q> Encode<'q, DB>,
{
    fn apply<'a>(&self, qb: &mut QueryBuilder<'a, DB>) {
        qb.push(" OFFSET ");
        qb.push_bind(self.0 as i64);
    }
}

impl<DB> SqlxFilter<DB> for OrderBy
where
    DB: Database,
{
    fn apply<'a>(&self, qb: &mut QueryBuilder<'a, DB>) {
        qb.push(" ORDER BY ");
        match self {
            OrderBy::Asc(id) => {
                qb.push(id.key());
                qb.push(" ASC");
            }
            OrderBy::Desc(id) => {
                qb.push(id.key());
                qb.push(" DESC");
            }
        }
    }
}

impl<DB, T> SqlxFilter<DB> for FromQueryFilter<T>
where
    DB: Database,
    T: SqlxFilter<DB> + Default + crate::common::WithFilterId + std::str::FromStr,
    i64: Type<DB> + for<'q> Encode<'q, DB>,
{
    fn apply<'a>(&self, qb: &mut QueryBuilder<'a, DB>) {
        self.inner.apply(qb);

        if let Some(order_by) = &self.order_by {
            order_by.apply(qb);
        }

        if let Some(limit) = &self.limit {
            limit.apply(qb);
        }

        if let Some(skip) = &self.skip {
            skip.apply(qb);
        }
    }
}