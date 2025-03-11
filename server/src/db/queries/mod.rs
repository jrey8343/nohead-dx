use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};
use sqlx::{Sqlite, SqlitePool};
use validator::Validate;

use super::Error;

pub mod todo;

/// ------------------------------------------------------------------------
/// # An BasicCrud trait to implement common CRUD methods on a database table
/// ------------------------------------------------------------------------
///
/// Implement the Model trait on a specific model to get a full set
/// of common CRUD functions: list, show, create, update, delete
///
/// # Example
///
/// ```rust
/// #[async_trait]
/// impl BasicCrud for Person {
///     type Id = i64;
///     type Record: Person;
///     type Changeset: PersonChangeset;
///
///     async fn list(db_pool: &DbPool) -> Result<Vec<Self::Record<'_>>, Error> {
///         // your implementation here
///         Ok(vec![])
///         }
///     // ...other methods
/// ```
///
/// ------------------------------------------------------------------------
#[async_trait]
pub trait BasicCrud {
    type Id: PartialOrd;
    type Record: Serialize + DeserializeOwned;
    type Changeset: Validate + DeserializeOwned;

    async fn load_all(
        executor: impl sqlx::Executor<'_, Database = Sqlite>,
    ) -> Result<Vec<Self::Record>, Error>;

    async fn load(
        id: Self::Id,
        executor: impl sqlx::Executor<'_, Database = Sqlite>,
    ) -> Result<Self::Record, Error>;

    async fn create(
        record: Self::Changeset,
        executor: impl sqlx::Executor<'_, Database = Sqlite>,
    ) -> Result<Self::Record, Error>;

    async fn create_batch(
        records: Vec<Self::Changeset>,
        db_pool: &SqlitePool,
    ) -> Result<Vec<Self::Record>, Error>;

    async fn update(
        id: Self::Id,
        record: Self::Changeset,
        executor: impl sqlx::Executor<'_, Database = Sqlite>,
    ) -> Result<Self::Record, Error>;

    async fn delete(
        id: Self::Id,
        executor: impl sqlx::Executor<'_, Database = Sqlite>,
    ) -> Result<Self::Record, Error>;

    async fn delete_batch(
        keys: Vec<Self::Id>,
        db_pool: &SqlitePool,
    ) -> Result<Vec<Self::Record>, Error>;
}
