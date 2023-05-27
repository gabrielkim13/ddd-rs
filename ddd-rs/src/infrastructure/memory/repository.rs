use std::collections::HashMap;

use crate::application::{ReadRepository, Repository};
use crate::domain::{AggregateRoot, Entity};

/// An in-memory implementation of [Repository], using a [HashMap].
///
/// # Examples
///
/// ```
/// use ddd_rs::application::{ReadRepository, Repository};
/// use ddd_rs::infrastructure::InMemoryRepository;
///
/// // By definition, only `AggregateRoot`s have repositories.
/// //
/// // Common entities must be retrieved and persisted through their associated aggregate roots.
/// #[derive(ddd_rs::AggregateRoot, ddd_rs::Entity, Clone)]
/// struct MyEntity {
///     #[entity(id)]
///     id: u32,
///     my_field: String,
/// }
///
/// impl MyEntity {
///     pub fn new(id: u32, my_field: impl ToString) -> Self {
///         Self {
///             id,
///             my_field: my_field.to_string(),
///         }
///     }
/// }
///
/// # tokio_test::block_on(async {
/// let repository: InMemoryRepository<MyEntity> = InMemoryRepository::new();
///
/// // Add some entities to the repository.
/// repository.add(MyEntity::new(1, "foo")).await.unwrap();
/// repository.add(MyEntity::new(2, "bar")).await.unwrap();
/// repository.add(MyEntity::new(3, "baz")).await.unwrap();
///
/// // Attempt to retrieve an entity by its ID.
/// let my_entity_2 = repository.get_by_id(2).await.unwrap();
///
/// assert!(my_entity_2.is_some());
/// assert_eq!(my_entity_2.as_ref().map(|e| e.my_field.as_str()), Some("bar"));
///
/// let mut my_entity_2 = my_entity_2.unwrap();
///
/// // Update the entity, then persist its changes.
/// my_entity_2.my_field = "qux".to_string();
///
/// let my_entity_2 = repository.update(my_entity_2).await.unwrap();
///
/// assert_eq!(my_entity_2.my_field.as_str(), "qux");
///
/// // Delete the entity permanently.
/// repository.delete(my_entity_2).await.unwrap();
///
/// // Assert it no longer exists.
/// assert!(!repository.exists(2).await.unwrap());
/// # })
/// ```
pub struct InMemoryRepository<T: AggregateRoot> {
    entities: std::sync::RwLock<HashMap<<T as Entity>::Id, T>>,
}

impl<T: AggregateRoot> InMemoryRepository<T> {
    /// Creates a new [InMemoryRepository].
    pub fn new() -> Self {
        Self {
            entities: std::sync::RwLock::new(HashMap::new()),
        }
    }
}

impl<T: AggregateRoot> Default for InMemoryRepository<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl<T: AggregateRoot + Clone> ReadRepository<T> for InMemoryRepository<T>
where
    <T as Entity>::Id: std::hash::Hash + Eq,
{
    async fn get_by_id(&self, id: <T as Entity>::Id) -> crate::Result<Option<T>> {
        let ro_entities = self.entities.read().unwrap();

        let entity = ro_entities.get(&id).cloned();

        Ok(entity)
    }

    async fn list(&self, skip: usize, take: usize) -> crate::Result<Vec<T>> {
        let ro_entities = self.entities.read().unwrap();

        let entities = ro_entities
            .values()
            .skip(skip)
            .take(take)
            .cloned()
            .collect();

        Ok(entities)
    }

    async fn count(&self) -> crate::Result<usize> {
        let ro_entities = self.entities.read().unwrap();

        Ok(ro_entities.len())
    }
}

#[async_trait::async_trait]
impl<T: AggregateRoot + Clone> Repository<T> for InMemoryRepository<T>
where
    <T as Entity>::Id: std::hash::Hash + Eq,
{
    async fn add(&self, entity: T) -> crate::Result<T> {
        let mut wo_entities = self.entities.write().unwrap();

        wo_entities.insert(entity.id().clone(), entity.clone());

        Ok(entity)
    }

    async fn update(&self, entity: T) -> crate::Result<T> {
        self.add(entity).await
    }

    async fn delete(&self, entity: T) -> crate::Result<()> {
        let mut wo_entities = self.entities.write().unwrap();

        wo_entities.remove(entity.id());

        Ok(())
    }
}
