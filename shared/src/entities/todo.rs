use serde::{Deserialize, Serialize};
use validator::Validate;

/// ------------------------------------------------------------------------
/// A shared Todo entity which can be used by both the client and the server.
/// ------------------------------------------------------------------------
#[derive(Serialize, Debug, Deserialize)]
pub struct Todo {
    /// The id of the record.
    pub id: i64,
    /// The description, i.e. what to do.
    pub description: String,
}

/// A changeset representing the data that is intended to be used to either create a new todo or update an existing todo.
///
/// Changesets are validatated in the [`create`] and [`update`] functions which return an [Result::Err] if validation fails.
///
/// Changesets can also be used to generate fake data for tests when the `test-helpers` feature is enabled:
///
/// ```
/// let todo_changeset: TodoChangeset = Faker.fake();
/// ```
#[cfg(feature = "test-helpers")]
use fake::{Dummy, faker::lorem::en::Sentence};

#[derive(Deserialize, Validate, Clone)]
#[cfg_attr(feature = "test-helpers", derive(Serialize, Dummy))]
pub struct TodoChangeset {
    /// The description must be at least 1 character long.
    #[cfg_attr(feature = "test-helpers", dummy(faker = "Sentence(3..8)"))]
    #[validate(length(min = 1, message = "Description must be at least 1 character long"))]
    pub description: String,
}
