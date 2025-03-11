use std::borrow::Cow;

use config::DatabaseConfig;
use sqlx::{Sqlite, SqlitePool, Transaction, sqlite::SqlitePoolOptions};
use tokio::sync::OnceCell;

pub use serde::de::DeserializeOwned;
pub use sqlx::test as db_test;
pub use validator::Validate;

pub mod queries;

/// ----------------------------------------------------------------
/// The main access point to the database in our #[server] functions
/// ----------------------------------------------------------------
///
/// ## Example
///
/// ```rust
/// sqlx::query!(".....")
/// .fetch_optional(&*DB)
/// .await?;
///```
///
pub static DB: Database = Database;

/// Custom migrator set to the correct path within the api testing environment
pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../migrations");

/// Starts a new database transaction.
///
/// Example:
/// ```
/// let tx = transaction(&app_state.db_pool).await?;
/// tasks::create(task_data, &mut *tx)?;
/// users::create(user_data, &mut *tx)?;
///
/// match tx.commit().await {
///     Ok(_) => Ok((StatusCode::CREATED, TasksView(results))),
///     Err(e) => Err((internal_error(e), "".into())),
/// }
/// ```
///
/// Transactions are rolled back automatically when they are dropped without having been committed.
pub async fn transaction() -> Result<Transaction<'static, Sqlite>, Error> {
    let tx = DB.begin().await?;

    Ok(tx)
}

/// Creates a connection pool to the database specified in the configuration.
/// Then creates a static reference to the pool.
static POOL: OnceCell<SqlitePool> = OnceCell::const_new();
pub async fn connect_pool(config: &DatabaseConfig) -> Result<(), Error> {
    let pool = SqlitePoolOptions::new()
        .connect(config.url.as_str())
        .await?;

    POOL.set(pool).expect("db already initialized");
    Ok(())
}

/// Wrapper around the SqlitePool singleton for convenient access in the application.
pub struct Database;
impl std::ops::Deref for Database {
    type Target = SqlitePool;

    fn deref(&self) -> &Self::Target {
        POOL.get()
            .expect("Database not initialized. Did you forget to call `db::connect()` in `main()`?")
    }
}

/// Errors that can occur as a result of a data layer operation.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// No record was found, e.g. when loading a record by ID. This variant is different from
    /// `Error::DbError(sqlx::Error::RowNotFound)` in that the latter indicates a bug, and
    /// `Error::NoRecordFound` does not. It merely originates from [sqlx::Executor::fetch_optional]
    /// returning `None`.
    #[error("no record found")]
    NoRecordFound,
    /// Return `422 Unprocessable Entity` on a unique constraint error.
    #[error("unique constraint error")]
    UniqueConstraint(Vec<(String, String)>),
    /// General database error, e.g. communicating with the database failed
    #[error("database query failed")]
    DatabaseError(#[from] sqlx::Error),
    /// An invalid changeset was passed to a writing operation such as creating or updating a record.
    #[error("validation failed")]
    ValidationError(#[from] validator::ValidationErrors),
    /// An error occurred while hashing a password.
    #[error("password hashing failed")]
    PasswordHashError(#[from] argon2::password_hash::Error),
}

/// ------------------------------------------------------------------------------------------
/// A little helper trait for more easily converting database constraint errors into API errors.
/// ------------------------------------------------------------------------------------------
/// ```rust,ignore
/// let user_id = sqlx::query_scalar!(
///     r#"insert into "user" (username, email, password_hash) values ($1, $2, $3) returning user_id"#,
///     username,
///     email,
///     password_hash
/// )
///     .fetch_one(&app_state.db)
///     .await
///     .on_constraint()?;
/// ```
pub trait ResultExt<T> {
    /// If `self` contains a SQLx database constraint error with the given name,
    /// transform the error.
    ///
    /// Otherwise, the result is passed through unchanged.
    fn map_constraint_err(self) -> Result<T, Error>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<Error>,
{
    fn map_constraint_err(self) -> Result<T, Error> {
        self.map_err(|e| match e.into() {
            Error::DatabaseError(sqlx::Error::Database(dbe))
                if dbe.code() == Some(Cow::Borrowed("2067")) =>
            {
                let (_, field) = dbe
                    .message()
                    .strip_prefix("UNIQUE constraint failed: ") // strip down to table.field
                    .and_then(|s| s.split_once('.'))
                    .unwrap_or_default(); // return an empty string if parsing fails

                Error::UniqueConstraint(vec![(field.to_string(), dbe.message().to_string())])
            }
            e => e, // Pass the error through unchanged if not a sqlx error
        })
    }
}
