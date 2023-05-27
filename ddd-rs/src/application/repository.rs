use crate::domain::{AggregateRoot, Entity};

/// Trait for representing a **Repository**.
///
/// > Therefore, use a Repository, the purpose of which is to encapsulate all the logic needed to
/// > obtain object references. The domain objects won’t have to deal with the infrastructure to get
/// > the needed references to other objects of the domain. They will just get them from the
/// > Repository and the model is regaining its clarity and focus.
///
/// # Example
///
/// See [InMemoryRepository](crate::infrastructure::memory::InMemoryRepository) for a sample
/// implementation of this trait.
#[async_trait::async_trait]
pub trait Repository<T: AggregateRoot>: ReadRepository<T> {
    /// Adds an entity to the repository.
    async fn add(&self, entity: T) -> crate::Result<T>;

    /// Updates an entity on the repository.
    async fn update(&self, entity: T) -> crate::Result<T>;

    /// Deletes the entity from the repository.
    async fn delete(&self, entity: T) -> crate::Result<()>;

    /// Adds the given entities to the repository.
    async fn add_range(&self, entities: Vec<T>) -> crate::Result<Vec<T>> {
        let mut added_entities = Vec::new();

        for entity in entities {
            self.add(entity).await.map(|e| added_entities.push(e))?;
        }

        Ok(added_entities)
    }

    /// Updates the given entities on the repository.
    async fn update_range(&self, entities: Vec<T>) -> crate::Result<Vec<T>> {
        let mut updated_entities = Vec::new();

        for entity in entities {
            self.update(entity)
                .await
                .map(|e| updated_entities.push(e))?;
        }

        Ok(updated_entities)
    }

    /// Deletes the given entities from the repository.
    async fn delete_range(&self, entities: Vec<T>) -> crate::Result<()> {
        for entity in entities {
            self.delete(entity).await?;
        }

        Ok(())
    }
}

/// Trait for representing a read-only **Repository**.
#[async_trait::async_trait]
pub trait ReadRepository<T: AggregateRoot>: Send + Sync {
    /// Gets an entity with the given ID.
    async fn get_by_id(&self, id: <T as Entity>::Id) -> crate::Result<Option<T>>;

    /// Lists all entities within a given page.
    async fn list(&self, skip: usize, take: usize) -> crate::Result<Vec<T>>;

    /// Returns the total number of entities in the repository.
    async fn count(&self) -> crate::Result<usize>;

    /// Checks whether an entity with the given ID exists in the repository.
    async fn exists(&self, id: <T as Entity>::Id) -> crate::Result<bool> {
        self.get_by_id(id).await.map(|e| e.is_some())
    }

    /// Checks if the repository is empty.
    async fn is_empty(&self) -> crate::Result<bool> {
        self.count().await.map(|c| c == 0)
    }
}
