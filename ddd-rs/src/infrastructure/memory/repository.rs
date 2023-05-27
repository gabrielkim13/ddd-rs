use std::collections::HashMap;

use crate::application::{ReadRepository, Repository};
use crate::domain::{AggregateRoot, Entity};

/// An in-memory implementation of [Repository], using a [HashMap].
///
/// See the example on [Repository] for usage information of this repository implementation.
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
