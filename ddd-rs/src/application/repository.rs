use std::sync::Arc;

use crate::domain::{AggregateRoot, AggregateRootEx, Entity};

use super::DomainEventHandler;

/// Trait for representing a **Repository**.
///
/// > Therefore, use a Repository, the purpose of which is to encapsulate all the logic needed to
/// > obtain object references. The domain objects won’t have to deal with the infrastructure to get
/// > the needed references to other objects of the domain. They will just get them from the
/// > Repository and the model is regaining its clarity and focus.
///
/// # Examples
///
/// This example uses the [InMemoryRepository](crate::infrastructure::memory::InMemoryRepository)
/// which is a sample implementation of this trait.
///
/// ```
/// use ddd_rs::{
///     application::{ReadRepository, Repository},
///     infrastructure::InMemoryRepository
/// };
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
///
/// See the [Repository] trait for the definition of a repository and a sample of its usage.
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

/// Repository extension abstraction, for performing operations over aggregates that implement the
/// [AggregateRootEx] trait.
///
/// # Examples
///
/// Building upon the [Repository] sample, this example shows how a repository object can be
/// extended in order to support concepts from the [AggregateRootEx] trait.
///
/// ```
/// use std::sync::Arc;
///
/// use ddd_rs::{
///     application::{DomainEventHandler, ReadRepository, Repository, RepositoryEx},
///     infrastructure::InMemoryRepository
/// };
///
/// // The aggregate below requires an action to be performed asynchronously, but doing so directly
/// // would require the aggregate root to:
/// //
/// // - Have a reference to one or many application services, thus breaching the isolation between
/// //   the Application and Domain layers;
/// // - Expect an async runtime, which is generally associated with I/O operations and
/// //   long-running tasks, to be available for the implementation of business rules.
/// //
/// // These can be seem as contrary to the modeling principles of DDD, since the domain model
/// // should be self-sufficient when enforcing its own business rules.
/// //
/// // Instead, the aggregate will register a domain event requesting the async action to be
/// // performed prior to being persisted to the repository.
/// #[derive(Clone, Debug, PartialEq)]
/// enum MyDomainEvent {
///     AsyncActionRequested { action: String },
/// }
///
/// #[derive(ddd_rs::AggregateRoot, ddd_rs::Entity, Clone)]
/// struct MyEntity {
///     #[entity(id)]
///     id: u32,
///     last_performed_action: Option<String>,
///     #[aggregate_root(domain_events)]
///     domain_events: Vec<MyDomainEvent>,
/// }
///
/// impl MyEntity {
///     pub fn new(id: u32) -> Self {
///         Self {
///             id,
///             last_performed_action: None,
///             domain_events: Default::default(),
///         }
///     }
///
///     pub fn request_async_action(&mut self, action: impl ToString) {
///         let domain_event = MyDomainEvent::AsyncActionRequested { action: action.to_string() };
///
///         self.register_domain_event(domain_event);
///     }
///
///     pub fn confirm_async_action_performed(&mut self, action: impl ToString) {
///         self.last_performed_action.replace(action.to_string());
///     }
/// }
///
/// // The domain event handler will usually be a context that holds references to all necessary
/// // services and providers to handle domain events.
/// struct MyDomainEventHandler {
///     repository: Arc<dyn Repository<MyEntity>>,
/// }
///
/// impl MyDomainEventHandler {
///     pub fn new(repository: Arc<dyn Repository<MyEntity>>) -> Self {
///         Self { repository }
///     }
/// }
///
/// #[async_trait::async_trait]
/// impl DomainEventHandler<MyEntity> for MyDomainEventHandler {
///     async fn handle(&self, mut entity: MyEntity, event: MyDomainEvent) -> ddd_rs::Result<MyEntity> {
///         let action = match event {
///             MyDomainEvent::AsyncActionRequested { action, .. } => action,
///         };
///
///         // Perform the async action...
///
///         entity.confirm_async_action_performed(action);
///
///         self.repository.update(entity).await
///     }
/// }
///
/// # tokio_test::block_on(async {
///
/// // Extend the basic repository to enable processing of domain events registered by the
/// // aggregate, upon persistence.
/// let repository = Arc::new(InMemoryRepository::new());
/// let domain_event_handler = Arc::new(MyDomainEventHandler::new(repository.clone()));
///
/// let repository_ex = RepositoryEx::new(domain_event_handler, repository);
///
/// // Create a new entity and request an async action.
/// let mut entity = MyEntity::new(42);
///
/// entity.request_async_action("foo");
///
/// // Assert that the action was not performed yet, but registered a domain event.
/// assert!(entity.last_performed_action.is_none());
/// assert_eq!(entity.domain_events.len(), 1);
///
/// // Persist the entity and assert that the action was performed as a result.
/// repository_ex.add(entity).await.unwrap();
///
/// let entity = repository_ex.get_by_id(42).await.unwrap().unwrap();
///
/// assert_eq!(entity.last_performed_action.unwrap(), "foo");
/// assert!(entity.domain_events.is_empty());
/// # })
/// ```
pub struct RepositoryEx<T: AggregateRootEx> {
    domain_event_handler: Arc<dyn DomainEventHandler<T>>,
    repository: Arc<dyn Repository<T>>,
}

impl<T: AggregateRootEx> RepositoryEx<T> {
    /// Creates a new instance of the extended repository.
    pub fn new(
        domain_event_handler: Arc<dyn DomainEventHandler<T>>,
        repository: Arc<dyn Repository<T>>,
    ) -> Self {
        Self {
            domain_event_handler,
            repository,
        }
    }

    async fn apply_domain_events(&self, mut entity: T) -> crate::Result<T> {
        let domain_events = entity.take_domain_events();

        let mut entity = entity;

        for event in domain_events {
            entity = self.domain_event_handler.handle(entity, event).await?;
        }

        Ok(entity)
    }
}

#[async_trait::async_trait]
impl<T: AggregateRootEx> ReadRepository<T> for RepositoryEx<T> {
    async fn get_by_id(&self, id: <T as Entity>::Id) -> crate::Result<Option<T>> {
        self.repository.get_by_id(id).await
    }

    async fn list(&self, skip: usize, take: usize) -> crate::Result<Vec<T>> {
        self.repository.list(skip, take).await
    }

    async fn count(&self) -> crate::Result<usize> {
        self.repository.count().await
    }
}

#[async_trait::async_trait]
impl<T: AggregateRootEx> Repository<T> for RepositoryEx<T> {
    async fn add(&self, entity: T) -> crate::Result<T> {
        let entity = self.repository.add(entity).await?;

        self.apply_domain_events(entity).await
    }

    async fn update(&self, entity: T) -> crate::Result<T> {
        let entity = self.repository.update(entity).await?;

        self.apply_domain_events(entity).await
    }

    async fn delete(&self, entity: T) -> crate::Result<()> {
        let entity = self.apply_domain_events(entity).await?;

        self.repository.delete(entity).await
    }
}
