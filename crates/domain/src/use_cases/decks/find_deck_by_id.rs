use utils::AppError;
use uuid::Uuid;

use crate::entities::Deck;
use crate::repositories::DecksRepository;

pub struct FindDeckByIdUseCase<T> {
    repository: Box<dyn DecksRepository<T>>,
}

impl<T> FindDeckByIdUseCase<T> {
    pub fn new(repository: Box<dyn DecksRepository<T>>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Deck, AppError> {
        let deck = self.repository.find_by_id(id).await?;

        Ok(deck)
    }
}
