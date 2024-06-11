use utils::AppError;

use crate::{entities::Deck, repositories::DecksRepository};

pub struct FindAllDecksUseCase<T> {
    repository: Box<dyn DecksRepository<T>>,
}

impl<T> FindAllDecksUseCase<T> {
    pub fn new(repository: Box<dyn DecksRepository<T>>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<Deck>, AppError> {
        let decks = self.repository.list_all().await?;

        Ok(decks)
    }
}
