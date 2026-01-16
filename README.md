# Filtrum

Filtrum is a flexible and type-safe filtering library for Rust, designed to simplify parsing complex query strings into structured filters and applying them to database queries. It provides seamless integration with popular frameworks like **Axum** and **SQLx**.

## Features

- **Standardized Query Parsing**: Parse query strings like `name[sw]=Ali&age[gte]=18&limit=10&order_by[desc]=age` into Rust structs.
- **Rich Filter Types**:
  - `NumberFilters`: Supports `eq`, `ne`, `gt`, `lt`, `gte`, `lte`.
  - `StringFilters`: Supports `eq`, `ne`, `like`, `not_like`, `sw` (starts with), `ew` (ends with), `co` (contains).
  - `EqualFilter`: Simple equality check for any type implementing `FromStr`.
- **Pagination & Sorting**: Built-in support for `limit`, `skip` (offset), and `order_by`.
- **Procedural Macro**: Use `#[derive(Filterable)]` to automatically generate parsing logic for your filter structs.
- **Framework Integrations**:
  - **Axum**: Extract filters directly from request query parameters.
  - **SQLx**: Automatically generate `WHERE`, `ORDER BY`, `LIMIT`, and `OFFSET` clauses for `QueryBuilder`.

## Installation

Add `filtrum` to your `Cargo.toml`:

```toml
[dependencies]
filtrum = { version = "0.1.0", features = ["derive", "axum", "sqlx"] }
```

Available features:
- `derive`: Enables the `Filterable` procedural macro.
- `axum`: Enables Axum `FromRequestParts` implementation for `FromQueryFilter`.
- `sqlx`: Enables SQLx `SqlxFilter` trait for applying filters to `QueryBuilder`.

## Quick Start

### 1. Define your filter struct

```rust
use filtrum::{Filterable, StringFilters, NumberFilters, EqualFilter};

#[derive(Default, Filterable)]
pub struct UserFilter {
    pub name: StringFilters,
    pub age: NumberFilters<i32>,
    pub active: EqualFilter<bool>,
}
```

### 2. Integration with Axum

Filtrum can automatically parse query parameters into your filter struct.

```rust
use filtrum::{FromQueryFilter, UserFilter};
use axum::{routing::get, Router};

async fn list_users(filter: FromQueryFilter<UserFilter>) -> String {
    format!("Filtering with: {:?}", filter)
}


### 3. Integration with SQLx

Use the `apply` method to append filter conditions to a `sqlx::QueryBuilder`.

```rust
use filtrum::sqlx::SqlxFilter;
use sqlx::{Postgres, QueryBuilder};

async fn fetch_users(pool: &sqlx::Pool<Postgres>, filter: FromQueryFilter<UserFilter>) {
    let mut qb = QueryBuilder::new("SELECT * FROM users WHERE 1=1");
    
    // This appends " AND name LIKE ... AND age >= ... ORDER BY ... LIMIT ... OFFSET ..."
    filter.apply(&mut qb);

    let query = qb.build();
    // ... execute query
}
```

## Supported Query Syntax

### String Filters
- `field=value` or `field[eq]=value`: Equality
- `field[ne]=value`: Inequality
- `field[sw]=value`: Starts with
- `field[ew]=value`: Ends with
- `field[co]=value`: Contains
- `field[like]=value`: SQL LIKE pattern

### Number Filters
- `field[eq]=value`: Equality
- `field[ne]=value`: Inequality
- `field[gt]=value`: Greater than
- `field[lt]=value`: Less than
- `field[gte]=value`: Greater than or equal
- `field[lte]=value`: Less than or equal

### Pagination & Sorting
- `limit=10`: Set result limit
- `skip=20`: Set result offset
- `order_by[asc]=field`: Sort ascending
- `order_by[desc]=field`: Sort descending

## Customizing the Derive Macro

The `#[filtrum]` attribute allows you to customize how fields are mapped to database columns.

```rust
#[derive(Default, Filterable)]
#[filtrum(table = "users")] // Optional prefix for all fields
pub struct UserFilter {
    #[filtrum(alias = "full_name")] // Map 'name' query param to 'full_name' column
    pub name: StringFilters,
    
    #[filtrum(skip)] // Skip this field in parsing/SQL generation
    pub internal_id: EqualFilter<i32>,

    #[filtrum(table = "profiles")] // Override table prefix for this field
    pub bio: StringFilters,
}
```

## License

MIT OR Apache-2.0
