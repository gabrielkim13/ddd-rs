use futures::future;

use crate::domain::{AggregateRoot, Entity};

/// Result type for [Repository] operations.
pub type Result<T, E = Box<dyn std::error::Error + Send + Sync>> = core::result::Result<T, E>;

/// Trait for representing a **Repository**.
///
/// > Therefore, use a Repository, the purpose of which is to encapsulate all the logic needed to
/// > obtain object references. The domain objects won’t have to deal with the infrastructure to get
/// > the needed references to other objects of the domain. They will just get them from the
/// > Repository and the model is regaining its clarity and focus.
///
/// # Examples
///
/// See [InMemoryRepository](crate::infrastructure::persistence::InMemoryRepository) for a sample
/// implementation of this trait.
#[async_trait::async_trait]
pub trait Repository<T: AggregateRoot>: ReadRepository<T> {
    /// Adds an entity to the repository.
    async fn add(&self, entity: T) -> Result<T>;

    /// Updates an entity on the repository.
    async fn update(&self, entity: T) -> Result<T>;

    /// Deletes the entity from the repository.
    async fn delete(&self, entity: T) -> Result<()>;

    /// Adds the given entities to the repository.
    async fn add_range(&self, entities: Vec<T>) -> Result<Vec<T>> {
        future::try_join_all(entities.into_iter().map(|e| self.add(e))).await
    }

    /// Updates the given entities on the repository.
    async fn update_range(&self, entities: Vec<T>) -> Result<Vec<T>> {
        future::try_join_all(entities.into_iter().map(|e| self.update(e))).await
    }

    /// Deletes the given entities from the repository.
    async fn delete_range(&self, entities: Vec<T>) -> Result<()> {
        future::try_join_all(entities.into_iter().map(|e| self.delete(e))).await?;

        Ok(())
    }
}

/// Trait for representing a read-only **Repository**.
#[async_trait::async_trait]
pub trait ReadRepository<T: AggregateRoot>: Send + Sync {
    /// Gets an entity with the given ID.
    async fn get_by_id(&self, id: <T as Entity>::Id) -> Result<Option<T>>;

    /// Lists all entities within a given page.
    async fn list(&self, skip: usize, take: usize) -> Result<Vec<T>>;

    /// Returns the total number of entities in the repository.
    async fn count(&self) -> Result<usize>;

    /// Returns a boolean whether the repository is not empty.
    async fn any(&self) -> Result<bool> {
        Ok(self.count().await? > 0)
    }
}
