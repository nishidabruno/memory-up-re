use anyhow::Result;
use utils::AppError;
use uuid::Uuid;

use crate::{entities::Flashcard, repositories::FlashcardsRepository};

pub struct FindFlashcardsByDeckIdUseCase<T> {
    repository: Box<dyn FlashcardsRepository<T>>,
}

impl<T> FindFlashcardsByDeckIdUseCase<T> {
    pub fn new(repository: Box<dyn FlashcardsRepository<T>>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Vec<Flashcard>, AppError> {
        let flashcard = self.repository.find_flashcard_by_deck_id(id).await?;

        Ok(flashcard)
    }
}
