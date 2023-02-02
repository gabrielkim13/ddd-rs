use std::marker::PhantomData;

use crate::application::{repository, DomainEventHandler, ReadRepository, Repository};
use crate::domain::{AggregateRoot, Entity};

/// A [Repository] implementation that handles [DomainEvent](crate::domain::DomainEvent)s when
/// persisting the [AggregateRoot] entity.
///
/// # Examples
///
/// ```
/// use std::sync::{Arc, RwLock};
///
/// use ddd_rs::application::{
///     domain_event_handler::{self, DomainEventHandler},
///     Repository,
/// };
/// use ddd_rs::domain::{AggregateRoot, Entity, UnitDomainEvent};
/// use ddd_rs::infrastructure::{DomainRepository, InMemoryRepository};
///
/// #[derive(ddd_rs::AggregateRoot, ddd_rs::Entity, Clone)]
/// struct MyEntity {
///     id: i32,
///     domain_events: Vec<UnitDomainEvent>,
///     created_at: chrono::DateTime<chrono::Utc>,
///     updated_at: chrono::DateTime<chrono::Utc>,
/// }
///
/// impl MyEntity {
///     pub fn new(id: i32) -> Self {
///         Self {
///             id,
///             domain_events: vec![],
///             created_at: chrono::Utc::now(),
///             updated_at: chrono::Utc::now(),
///         }
///     }
/// }
///
/// struct MockDomainEventHandler {
///     calls: Arc<RwLock<i32>>,
/// }
///
/// impl MockDomainEventHandler {
///     pub fn new(calls: Arc<RwLock<i32>>) -> Self {
///         Self { calls }
///     }
/// }
///
/// #[async_trait::async_trait]
/// impl DomainEventHandler<UnitDomainEvent> for MockDomainEventHandler {
///     async fn handle(&self, _event: UnitDomainEvent) -> domain_event_handler::Result<()> {
///         let mut calls = self.calls.write().unwrap();
///         *calls += 1;
///
///         Ok(())
///     }
/// }
///
/// # tokio_test::block_on(async {
/// let calls = Arc::new(RwLock::new(0));
///
/// let my_entity_repository = DomainRepository::new(
///     InMemoryRepository::new(),
///     MockDomainEventHandler::new(calls.clone()),
/// );
///
/// let mut my_entity = MyEntity::new(1);
///
/// my_entity.register_domain_event(UnitDomainEvent);
/// my_entity.register_domain_event(UnitDomainEvent);
/// my_entity.register_domain_event(UnitDomainEvent);
///
/// my_entity_repository.add(my_entity).await.unwrap();
///
/// let calls = *calls.read().unwrap();
///
/// assert_eq!(calls, 3);
/// # })
/// ```
pub struct DomainRepository<
    T: AggregateRoot + Send + 'static,
    TRepository: Repository<T>,
    TDomainEventHandler: DomainEventHandler<<T as AggregateRoot>::DomainEvent>,
> {
    aggregate_root_type: PhantomData<T>,
    repository: TRepository,
    domain_event_handler: TDomainEventHandler,
}

impl<
        T: AggregateRoot,
        TRepository: Repository<T>,
        TDomainEventHandler: DomainEventHandler<<T as AggregateRoot>::DomainEvent>,
    > DomainRepository<T, TRepository, TDomainEventHandler>
{
    /// Creates a new [DomainRepository].
    pub fn new(repository: TRepository, domain_event_handler: TDomainEventHandler) -> Self {
        Self {
            aggregate_root_type: PhantomData,
            repository,
            domain_event_handler,
        }
    }
}

#[async_trait::async_trait]
impl<
        T: AggregateRoot,
        TRepository: Repository<T>,
        TDomainEventHandler: DomainEventHandler<<T as AggregateRoot>::DomainEvent>,
    > Repository<T> for DomainRepository<T, TRepository, TDomainEventHandler>
{
    async fn add(&self, mut entity: T) -> repository::Result<T> {
        let domain_events = entity.drain_domain_events();

        let result = self.repository.add(entity).await?;

        for event in domain_events.into_iter() {
            self.domain_event_handler.handle(event).await?;
        }

        Ok(result)
    }

    async fn update(&self, mut entity: T) -> repository::Result<T> {
        let domain_events = entity.drain_domain_events();

        let result = self.repository.update(entity).await?;

        for event in domain_events.into_iter() {
            self.domain_event_handler.handle(event).await?;
        }

        Ok(result)
    }

    async fn delete(&self, mut entity: T) -> repository::Result<()> {
        let domain_events = entity.drain_domain_events();

        let result = self.repository.delete(entity).await?;

        for event in domain_events.into_iter() {
            self.domain_event_handler.handle(event).await?;
        }

        Ok(result)
    }
}

#[async_trait::async_trait]
impl<
        T: AggregateRoot,
        TRepository: Repository<T>,
        TDomainEventHandler: DomainEventHandler<<T as AggregateRoot>::DomainEvent>,
    > ReadRepository<T> for DomainRepository<T, TRepository, TDomainEventHandler>
{
    async fn get_by_id(&self, id: <T as Entity>::Id) -> repository::Result<Option<T>> {
        self.repository.get_by_id(id).await
    }

    async fn list(&self, skip: usize, take: usize) -> repository::Result<Vec<T>> {
        self.repository.list(skip, take).await
    }

    async fn count(&self) -> repository::Result<usize> {
        self.repository.count().await
    }
}
