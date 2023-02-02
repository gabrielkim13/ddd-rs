use std::collections::HashMap;

use crate::application::{repository, ReadRepository, Repository};
use crate::domain::{AggregateRoot, Entity};

/// An in-memory implementation of [Repository], using a [HashMap].
///
/// # Examples
///
/// ```
/// use ddd_rs::application::{ReadRepository, Repository};
/// use ddd_rs::domain::{AggregateRoot, Entity, UnitDomainEvent};
/// use ddd_rs::infrastructure::InMemoryRepository;
///
/// #[derive(ddd_rs::AggregateRoot, ddd_rs::Entity, Clone)]
/// struct MyEntity {
///     id: i32,
///     my_field: String,
///     domain_events: Vec<UnitDomainEvent>,
///     created_at: chrono::DateTime<chrono::Utc>,
///     updated_at: chrono::DateTime<chrono::Utc>,
/// }
///
/// impl MyEntity {
///     pub fn new(id: i32, my_field: impl ToString) -> Self {
///         Self {
///             id,
///             my_field: my_field.to_string(),
///             domain_events: vec![],
///             created_at: chrono::Utc::now(),
///             updated_at: chrono::Utc::now(),
///         }
///     }
/// }
///
/// # tokio_test::block_on(async {
/// let my_entity_repository: InMemoryRepository<MyEntity> = InMemoryRepository::new();
///
/// my_entity_repository.add(MyEntity::new(1, "foo")).await.unwrap();
/// my_entity_repository.add(MyEntity::new(2, "bar")).await.unwrap();
/// my_entity_repository.add(MyEntity::new(3, "baz")).await.unwrap();
///
/// let my_entity_2 = my_entity_repository.get_by_id(2).await.unwrap();
///
/// assert!(my_entity_2.is_some());
/// assert_eq!(my_entity_2.map(|e| e.my_field), Some(String::from("bar")));
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
    async fn get_by_id(&self, id: <T as Entity>::Id) -> repository::Result<Option<T>> {
        let ro_entities = self.entities.read().unwrap();

        let entity = ro_entities.get(&id).cloned();

        Ok(entity)
    }

    async fn list(&self, skip: usize, take: usize) -> repository::Result<Vec<T>> {
        let ro_entities = self.entities.read().unwrap();

        let entities = ro_entities
            .values()
            .skip(skip)
            .take(take)
            .cloned()
            .collect();

        Ok(entities)
    }

    async fn count(&self) -> repository::Result<usize> {
        let ro_entities = self.entities.read().unwrap();

        Ok(ro_entities.len())
    }
}

#[async_trait::async_trait]
impl<T: AggregateRoot + Clone> Repository<T> for InMemoryRepository<T>
where
    <T as Entity>::Id: std::hash::Hash + Eq,
{
    async fn add(&self, entity: T) -> repository::Result<T> {
        let mut wo_entities = self.entities.write().unwrap();

        wo_entities.insert(entity.id(), entity.clone());

        Ok(entity)
    }

    async fn update(&self, entity: T) -> repository::Result<T> {
        self.add(entity).await
    }

    async fn delete(&self, entity: T) -> repository::Result<()> {
        let mut wo_entities = self.entities.write().unwrap();

        wo_entities.remove(&entity.id());

        Ok(())
    }
}
