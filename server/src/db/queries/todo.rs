use async_trait::async_trait;
use shared::entities::todo::{Todo, TodoChangeset};

use super::BasicCrud;

#[async_trait]
impl BasicCrud for Todo {
    type Id = i64;

    type Record = Todo;

    type Changeset = TodoChangeset;

    async fn load_all(
        executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    ) -> Result<Vec<Self::Record>, crate::db::Error> {
        todo!()
    }

    async fn load(
        id: Self::Id,
        executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    ) -> Result<Self::Record, crate::db::Error> {
        todo!()
    }

    async fn create(
        record: Self::Changeset,
        executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    ) -> Result<Self::Record, crate::db::Error> {
        todo!()
    }

    async fn create_batch(
        records: Vec<Self::Changeset>,
        db_pool: &sqlx::SqlitePool,
    ) -> Result<Vec<Self::Record>, crate::db::Error> {
        todo!()
    }

    async fn update(
        id: Self::Id,
        record: Self::Changeset,
        executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    ) -> Result<Self::Record, crate::db::Error> {
        todo!()
    }

    async fn delete(
        id: Self::Id,
        executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    ) -> Result<Self::Record, crate::db::Error> {
        todo!()
    }

    async fn delete_batch(
        keys: Vec<Self::Id>,
        db_pool: &sqlx::SqlitePool,
    ) -> Result<Vec<Self::Record>, crate::db::Error> {
        todo!()
    }
}
