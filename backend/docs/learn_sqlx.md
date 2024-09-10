# Learn SQLX

## References

- [SQLx is an async, pure Rust† SQL crate featuring compile-time checked queries without a DSL](https://github.com/launchbadge/sqlx)
- [Feature flags](https://docs.rs/crate/sqlx/latest/features)
- [SQLx is my favorite PostgreSQL driver to use with Rust.](https://www.youtube.com/watch?v=TCERYbgvbq0)

## How to do query verification using sqlx

- Make sure `sqlx-cli` is installed, otherwise run: `cargo install sqlx-cli`.
- Set up `DATABASE_URL` like: `export DATABASE_URL="postgres://postgres:postgres@localhost:5432/myapp"`
- Run `cargo sqlx prepare`

## How to handle option type

- [A Brief Introduction about Rust SQLx](https://medium.com/@edandresvan/a-brief-introduction-about-rust-sqlx-5d3cea2e8544)
