use anyhow::Result;
use utils::AppError;
use uuid::Uuid;

use crate::repositories::FlashcardsRepository;

pub struct GetTotalFlashcardsByDeckUseCase<T> {
    repository: Box<dyn FlashcardsRepository<T>>,
}

impl<T> GetTotalFlashcardsByDeckUseCase<T> {
    pub fn new(repository: Box<dyn FlashcardsRepository<T>>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, deck_id: Uuid) -> Result<i32, AppError> {
        let total_flashcards = self
            .repository
            .get_total_flashcards_by_deck(deck_id)
            .await?;

        Ok(total_flashcards)
    }
}
