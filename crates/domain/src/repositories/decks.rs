use async_trait::async_trait;

use utils::AppError;
use uuid::Uuid;

use crate::entities::Deck;

#[async_trait]
// trait object implementation needs to be Send + Sync
pub trait DecksRepository<T>: Send + Sync {
    fn new(pool: T) -> Self
    where
        T: Send + Sync,
        Self: Sized;
    async fn save(&self, deck: &Deck) -> Result<(), AppError>;
    async fn list_all(&self) -> Result<Vec<Deck>, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Deck, AppError>;
}
